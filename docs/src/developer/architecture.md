# Project Architecture

**Kanji Master** is built on a modular architecture that clearly separates **data logic (Backend)** and **presentation (UI)**. This facilitates maintenance, feature expansion, and testing.

---

## Tech Stack

*   **Language**: Rust (Edition 2024)
*   **Graphics Engine**: `eframe` (wrapper over `egui`) — Immediate Mode GUI.
*   **Database**: SQLite (via the `rusqlite` crate).
*   **Serialization**: `serde` + `serde_json` / `toml`.
*   **Resource Embedding**: `rust-embed` (for DB, JSON, fonts, and SVG).

---

## Module Structure

### `back` (Backend)

Contains **business logic** independent of the GUI:

*   **`core.rs`** — working with SQLite (`core.db`), `Kanji` structures, and data retrieval methods.
*   **`config.rs`** — configuration management (`config.toml`): loading, saving, defining OS-dependent paths.
*   **`localization.rs`** — structures and functions for JSON localization files.
*   **`cards.rs`** — logic for practice sessions: card queue, shuffling, flip status.
*   **`romaji_kana.rs`** — text conversion from Latin script to hiragana/katakana.

### `ui` (Interface)

Responsible for **rendering and handling user input**:

*   **`interface.rs`** — the main application loop (`impl eframe::App`), screen routing (`enum Screen`), and layout of all panels.
*   **`settings.rs`** — the settings window and management of user preferences.
*   **`animator.rs`** — component for rendering SVG kanji stroke order animations.

---

## Database (`core.db`)

The application uses a **relational SQLite database**. Main tables:

1.  **`kanji`** — core information (id, character, JLPT, grade, strokes).
2.  **`read`** — kanji readings (on'yomi and kun'yomi).
3.  **`examples`** — example words and phrases containing the kanji.

> The data is joined in Rust via a single large JOIN query at startup and then held in memory for fast search and filtering.

---

## Resources and `Asset`

Using **`rust-embed`**, all static files (DB, JSON, fonts, SVG) are embedded into the binary.

*   On the first launch (`main.rs -> initialize_app_data`), resources are **extracted to the user's system configuration folder**:
    *   Windows: `C:\Users\<Username>\AppData\Roaming\KanjiMaster\`
    *   Linux: `/home/<username>/.config/kanjimaster/`
    *   macOS: `/Users/<Username>/Library/Application Support/KanjiMaster/`

*   This ensures **application autonomy** and allows the user to easily reset settings or change localization.