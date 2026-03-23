# Code Internals

This document is a technical guide for developers who want to modify the **Kanji Master** core.
It covers architectural decisions, state management, data flow, and specifics of working with **Immediate Mode GUI** in Rust.

---

## 1. Application Lifecycle

Kanji Master operates as a **hybrid of a utility and a game engine**, rather than a standard CRUD application.

### Initialization (`main.rs`)

The entry point is the `main` function. It performs critical tasks before the interface is rendered:

1.  **Resource Extraction (`initialize_app_data`)**:
    *   The application is compiled into a **single binary file**, including the SQLite database `core.db`, JSON localization files, and SVG resources using the `rust-embed` crate.
    *   *Logic*: On startup, it checks for the existence of the system configuration folder (e.g., `%APPDATA%\KanjiMaster` on Windows or `/home/<user>/.config/kanjimaster/` on Linux). If the files are missing—they are extracted from the binary and written to disk.
    *   *Why*: Guarantees the presence of valid data, and the user can reset settings by deleting the configs.

2.  **Loading Configuration (`load_config`)**:
    *   Attempts to read `config.toml`.
    *   If the file is missing or corrupted, a default configuration is created (`Config::default()`).

3.  **Launching the GUI (`eframe::run_native`)**:
    *   Initializes the OpenGL/Vulkan/Metal context.
    *   Creates the main state object — `App`.

---

## 2. Monolithic State (`App`)

All application logic is managed through a **single state structure** `App` (`interface.rs`), which is a standard pattern for `egui`.

```rust
struct App {
    // 1. Navigation
    tab_manager: TabManager,

    // 2. "Cold" data (Read-only)
    kanji: Vec<Arc<Kanji>>,

    // 3. "Hot" data (Mutable)
    config: Config,
    settings: Settings,
    paths: Paths,
    localization: Localization,

    // 4. Subsystems
    translate_state: TranslateState,
    recognition: RecognitionSystem,
    svg_cache: SvgCache,
}
```

### Key Points on Memory Management

*   **In-Memory Database**: `self.kanji` is loaded once at startup. ~2-3 thousand objects take only a few megabytes of RAM — this speeds up searching and filtering without disk queries.
*   **Cloning**: `kanji.clone()` is used instead of complex schemes with `Rc/RefCell` to simplify working with the Borrow Checker.

---

## 3. Immediate Mode GUI and the `update` Cycle

Kanji Master uses **egui (Immediate Mode GUI)**. The interface is completely rebuilt every frame.

### Core Principles

1.  The `update(&mut self, ctx, frame)` method is called on each frame (e.g., on mouse movement or at 60 FPS for animations).
2.  **No stored UI objects** — all buttons and windows are recreated each frame.
3.  **Events** are checked immediately during rendering:

```rust
if ui.button("Click me").clicked() {
    self.counter += 1;
}
```

### UI Architecture (`interface.rs`)

```rust
fn update(...) {
    // 1. Global styles (fonts, sizes)
    // 2. TopBar (always visible)
    self.top_bar(ctx);

    // 3. Central panel
    egui::CentralPanel::default().show(ctx, |ui| {
        match &self.current_screen {
            Screen::Home => self.home_screen(ui),
            Screen::Kanji(k) => self.kanji_screen(ui, k),
            // ... routing for other screens
        }
    });

    // 4. Overlay windows (Settings)
    if self.settings.setting(...) {
        self.save();
    }
}
```

---

## 4. Data Subsystem (`back/core.rs`)

Working with SQLite (`rusqlite`) is encapsulated in the `Database` struct.

*   **SQL Query**: One large JOIN is executed between the `kanji`, `read`, and `examples` tables.
*   **NULL Handling**: Via `Option<String>`, mapped to empty strings for the UI.
*   **Sorting**: Data is sorted at the Rust vector level for predictable display.

---

## 5. Localization

*   **Dynamic Switching**: `reload_interface_localization` replaces `self.localization`. The GUI updates instantly.
*   **Two Layers**:
    1.  `localization.json` — interface.
    2.  `kanji-localization.json` — kanji and example translations.

---

## 6. Flashcard Subsystem (`back/cards.rs`)

*   **Session State Machine**:
    *   `queue`: A shuffled `Vec<Kanji>` for studying.
    *   `current_index`: The current card.
    *   `is_card_flipped`: The flipped state of the card.
*   **Session Creation**: Filtering by JLPT/deck, shuffling, card limit.
*   **Lifecycle**: Stored in `Option<CardsSession>` inside `App`.

---

## 7. Practical Guide: Adding New Features

### Example: Adding a "Dark Mode" Parameter

1.  **Backend (`config.rs`)**: Add `pub dark_mode: bool` to `Config`.
2.  **State (`settings.rs`)**: Add a field to `Settings`.
3.  **UI**: Add a checkbox in the `setting(...)` method.
4.  **Logic (`interface.rs`)**: Apply the value during `update` and save it to `config`.

### Example: Adding a New Screen

1.  Add a variant to the `enum Screen`.
2.  Implement a rendering method in `App`.
3.  Add handling in `update` for routing.

---

## 8. Common Issues and Nuances

1.  **Borrow Checker in UI**: Avoid mutating `self` inside `ui.show(|ui| { ... })`.
2.  **Performance**: Execute heavy operations during initialization or in a separate thread, not in `update`.
3.  **Input Focus**: Managed using `ui.memory(|m| m.request_focus(id))`.