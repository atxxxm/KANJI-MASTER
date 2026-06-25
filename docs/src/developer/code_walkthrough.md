# Code Internals

A technical guide for developers who want to understand or modify Kanji Master. Covers state management, data flow, and egui patterns used throughout the codebase.

---

## 1. Application Lifecycle

### Initialization (`main.rs`)

```
main()
  ├─ initialize_app_data()   — extract embedded files to config dir if missing
  ├─ load_config()           — parse config.toml or write default
  └─ eframe::run_native()    — start the OpenGL window, create App
```

`initialize_app_data` iterates over all files embedded via `rust-embed` and writes them to the system config directory only if they don't already exist. This means user-modified files (translations, config) are never overwritten on update.

---

## 2. Application State (`app.rs`)

All runtime state lives in one struct:

```rust
struct App {
    tab_manager: TabManager,       // open tabs and active index

    kanji: Vec<Arc<Kanji>>,        // full DB, loaded once at startup
    kanji_by_id: HashMap<i32, Arc<Kanji>>,  // O(1) lookup by id

    config: Config,
    paths: Paths,
    settings: Settings,
    localization: Localization,
    translate_state: TranslateState,

    recognition: RecognitionSystem,
    svg_cache: SvgCache,

    settings_window: bool,
    kanji_loc_setup: bool,         // show first-run localization screen
    error_notification: Option<String>,
    applied_style_key: Option<(u32, bool)>,  // avoid rebuilding style every frame
}
```

`kanji` is loaded once and held in memory for the lifetime of the app. At ~2–3k entries it takes only a few MB. All search and filtering is a simple Vec iteration — no SQL at runtime.

---

## 3. Tab System (`tabs.rs`)

Navigation is tab-based. Each tab carries its own state:

```rust
enum TabType {
    Home(HomeState),
    KanjiList(KanjiListState),
    KanjiDetail(KanjiDetailState),
    Kana(bool),
    RomajiToKana(RomajiKanaState),
    Translate(TranslateTabState),
    AnkiExport(AnkiExportState),
    DrawSearch(DrawSearchState),
}
```

`TabManager` holds `Vec<Tab>` and the active index. Tabs can be:
- Opened in the foreground (`add_tab(..., true)`)
- Opened in the background (`add_tab(..., false)`) — useful for middle-click on kanji cards
- Reordered by drag-and-drop
- Closed via `×` button or middle-click

---

## 4. Rendering and Context (`app.rs`, `context.rs`)

The `update()` method runs every frame:

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // 1. Apply style only when font size or theme actually changed
    if self.applied_style_key != Some(style_key) { ... }

    // 2. Top bar (always visible)
    self.top_bar(ctx);

    // 3. Tab bar + active view
    egui::CentralPanel::default().show(ctx, |ui| {
        self.tab_manager.ui(ui, ctx, &self.localization);
        // route to the correct view based on active tab
        match content {
            TabType::Home(state) => views::home::render(ui, state, &context),
            TabType::KanjiList(state) => views::kanji_list::render(ui, state, &context),
            // ...
        }
    });

    // 4. Overlay windows
    self.kanji_localization_setup(ctx);
    self.settings.setting(&mut self.settings_window, &mut self.paths, ctx);
}
```

Each view receives an `AppContext` — a short-lived struct of references that avoids passing a dozen individual arguments:

```rust
pub struct AppContext<'a> {
    pub kanji: &'a Vec<Arc<Kanji>>,
    pub kanji_by_id: &'a HashMap<i32, Arc<Kanji>>,
    pub config: &'a mut Config,
    pub settings: &'a Settings,
    pub localization: &'a Localization,
    pub translate_state: &'a mut TranslateState,
    pub svg_cache: &'a mut SvgCache,   // mut for lazy SVG loading
}
```

Views return `Option<(TabType, TabOpenMode)>` to request opening a new tab without holding a mutable reference to `TabManager` during rendering.

---

## 5. SVG Cache and Recognition (`svg_cache.rs`, `recognition.rs`)

### Lazy SVG Cache

`SvgCache` stores a `HashMap<i32, String>` (id → unicode) at startup. On first access for a given kanji, it reads and parses the SVG file, flattens bezier curves into point sequences, and stores the result. Subsequent accesses are instant.

### Recognition Pipeline

1. At startup, `RecognitionSystem::load_from_svgs()` reads all SVGs, resamples each stroke to 32 uniformly-spaced points, and stores `SimplifiedKanji { id, strokes }`.
2. When the user clicks Search in Draw & Search, their strokes are normalized the same way.
3. Candidates are filtered by stroke count (±2 tolerance).
4. Each candidate is scored via `calculate_similarity()`:
   - **DTW (Dynamic Time Warping)** measures the distance between each pair of strokes, handling non-uniform drawing speed.
   - **Greedy matching** pairs each user stroke to the closest unmatched template stroke — order-independent.

---

## 6. Localization System

Two separate layers:

| File | Controls |
|---|---|
| `localization.toml` | All UI strings (buttons, labels, tooltips) |
| `en.json` / `ru.json` / custom | Kanji meanings and example translations |

Switching the UI language calls `reload_interface_localization()`, which replaces `self.localization` and the interface updates on the next frame.

The kanji translation file is loaded into `TranslateState.data` as a `HashMap<String, KanjiTranslation>` keyed by the kanji character.

---

## 7. Adding New Features

### New config option

1. Add field to `Config` in `config.rs` with a `#[serde(default = ...)]` fallback.
2. Add field to `Settings` in `settings.rs`.
3. Add UI control in `Settings::setting()`.
4. Apply in `app.rs` `update()` and call `self.save()`.

### New tab / screen

1. Add a state struct (e.g. `MyFeatureState`) to `tabs.rs`.
2. Add a variant to `TabType`.
3. Add a title in `TabType::title()`.
4. Create `src/ui/views/my_feature.rs` with a `render()` function.
5. Add the arm to the `match content { ... }` block in `app.rs`.

### New backend module

1. Create `src/back/my_module.rs`.
2. Add `pub mod my_module;` to `src/back/mod.rs`.
3. Add the field to `App` and initialize in `App::new()`.

---

## 8. Common egui Patterns and Gotchas

**Borrow checker in closures:** `ui.show(|ui| { ... })` borrows `ui` mutably. You cannot also borrow `self` mutably inside. Use the `AppContext` pattern or extract values before the closure.

**Style rebuilding:** Calling `ctx.set_style()` every frame is expensive. Kanji Master tracks `applied_style_key: Option<(u32, bool)>` (font size bits + dark mode flag) and only rebuilds when it changes.

**Tab actions:** Views cannot call `tab_manager.add_tab()` directly while `tab_manager` is borrowed for rendering. They return `Option<(TabType, TabOpenMode)>` and the parent handles it after the closure ends.

**Focus management:** Use `ui.memory(|m| m.has_focus(id))` to check and `response.request_focus()` to set focus — for example, the home search bar auto-focuses when the tab opens.
