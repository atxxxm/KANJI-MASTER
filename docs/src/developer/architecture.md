# Project Architecture

**Kanji Master** is built on a modular architecture that clearly separates **data logic (backend)** and **presentation (UI)**. This makes it easy to add features, fix bugs, and reason about state.

---

## Tech Stack

| Component | Library |
|---|---|
| Language | Rust (Edition 2024) |
| GUI | `eframe` / `egui` — Immediate Mode GUI |
| Database | SQLite via `rusqlite` |
| Serialization | `serde` + `serde_json` + `toml` |
| Resource Embedding | `rust-embed` |
| File Dialogs | `rfd` |
| SVG Parsing | `roxmltree`, `svgtypes`, `kurbo` |

---

## Module Structure

### `back/` — Backend

Contains all business logic, independent of the GUI:

| File | Responsibility |
|---|---|
| `core.rs` | `Database` struct, `Kanji` struct, SQLite queries |
| `config.rs` | `Config` struct, `load_config`, `save_config`, system paths |
| `localization.rs` | TOML UI localization loading and `Localization` struct |
| `translation.rs` | JSON kanji translation loading, `TranslateState`, save/load |
| `recognition.rs` | Handwritten kanji recognition (DTW + greedy stroke matching) |
| `svg_cache.rs` | Lazy SVG parsing and caching for stroke order animations |
| `romaji_kana.rs` | Romaji → Hiragana/Katakana conversion |

### `ui/` — Interface

Responsible for rendering and handling user input:

| File | Responsibility |
|---|---|
| `app.rs` | Main `App` struct, `eframe::App` impl, top bar, update loop |
| `tabs.rs` | `TabManager`, `TabType` enum, all tab state structs |
| `context.rs` | `AppContext` — short-lived bundle of references passed to views |
| `settings.rs` | Settings window, file dialogs, localization import |
| `animator.rs` | SVG stroke order animation component |
| `theme.rs` | Color palette, `visuals()` for light/dark themes |
| `views/` | One file per screen: `home`, `kanji_list`, `kanji_detail`, `kana`, `draw_search`, `anki_export`, `translate`, `romaji_kana` |

---

## Database Schema (`core.db`)

Three tables joined at startup into an in-memory `Vec<Arc<Kanji>>`:

```
kanji    — id, kanji, strokes, jlpt, grade, frequency, unicode
read     — kanji_id, onyomi, onyomi_romaji, kunyomi, kunyomi_romaji
examples — kanji_id, example
```

The full join is executed once at launch. All search and filtering operates on the in-memory vector — no runtime SQL queries.

---

## SVG Cache (Lazy Loading)

Stroke order animations are stored as SVG files in `kanji-svg/`. The cache is **lazy**:

- At startup, `SvgCache::prepare()` only stores a `kanji_id → unicode` mapping — no files are parsed.
- When a kanji detail tab is first opened, `SvgCache::get_or_load()` parses the SVG on demand and caches the result.
- `RecognitionSystem` loads its own simplified stroke data independently via `load_from_svgs()`, so draw-search works immediately without waiting for the UI cache.

---

## Data Flow on Startup

```
main()
  └─ initialize_app_data()      — extract embedded assets to config dir
  └─ load_config()              — read config.toml (or create default)
  └─ eframe::run_native()
       └─ App::new()
            ├─ load Localization (TOML)
            ├─ Database::get_kanji() → Vec<Arc<Kanji>>
            ├─ SvgCache::prepare()   — build unicode map, no SVG parsing
            └─ RecognitionSystem::load_from_svgs() — parse SVGs for recognition
```

---

## Resources and Embedding

All static assets (database, localization files, fonts, SVGs) are embedded into the binary via `rust-embed`.

On first launch, `initialize_app_data()` extracts missing files to the system config directory:

- **Windows:** `C:\Users\<Username>\AppData\Roaming\atom\kanjimaster\config\`
- **Linux:** `/home/<username>/.config/atom/kanjimaster/`
- **macOS:** `/Users/<Username>/Library/Application Support/atom.kanjimaster/`

Existing files are never overwritten, so user edits (custom translations, config) are preserved across updates.
