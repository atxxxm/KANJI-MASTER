# 漢字 Kanji Master

**Kanji Master** is a free, offline, cross-platform desktop app for learning and looking up Japanese kanji — built with **Tauri** (Rust backend) and **React**.

It started as a personal study tool and was opened up so anyone can use it, tweak it, or build on it.

---

## Screenshots

| Home | Kanji list |
|---|---|
| ![Home](assets/1.png) | ![Kanji list](assets/2.png) |

| Kanji detail (stroke animation) | Romaji → Kana |
|---|---|
| ![Kanji detail](assets/3.png) | ![Romaji to Kana](assets/4.png) |

---

## Features

- **Home** — a random "kanji of the session" plus a quick search across the whole dictionary.
- **Kanji list** — browse every kanji, filter by JLPT level (N5–N1), search by character, reading, or meaning.
- **Kanji detail** — stroke-order animation drawn live on canvas, on'yomi/kun'yomi readings, meaning, and usage examples with clickable kanji.
- **Kana chart** — the full hiragana/katakana gojūon, dakuten, handakuten, and yōon tables.
- **Romaji → Kana** — live romaji-to-kana conversion as you type, with a kanji assistant panel suggesting matching kanji for the current reading.
- **Draw Search** — draw a kanji with your mouse or stylus and get the closest matches from the dictionary (stroke-order-independent DTW matching), with undo/redo.
- **Translate Kanji** — a guided editor for writing your own meanings and example translations, saved to a portable JSON file.
- **Anki Export** — select kanji, pick which fields to include, and export a tab-separated file ready to import into [Anki](https://apps.ankiweb.net/).
- **Tabs** — every tool opens in its own closable, reorderable (drag-and-drop) tab, with state preserved when you switch away and back. Right-click a tab or a kanji card for more options ("Open in new tab", "Close others", etc.).
- **Keyboard shortcuts** — `Ctrl+T` new tab, `Ctrl+W` close tab, `Ctrl+F` focus the kanji search, `Escape` clear it.
- **Settings** — dark/light theme, interface and kanji font size, stroke-animation speed, and custom paths for the database, kanji SVG data, and your localization file.

---

## Tech stack

- **Backend:** Rust, [Tauri 2](https://tauri.app/), [rusqlite](https://github.com/rusqlite/rusqlite), [serde](https://serde.rs/), [kurbo](https://github.com/linebender/kurbo) / [roxmltree](https://github.com/RazrFalcon/roxmltree) for SVG stroke parsing.
- **Frontend:** React, TypeScript, Vite, [Framer Motion](https://www.framer.com/motion/).
- **Data:** SQLite for the kanji dictionary, KanjiVG SVGs for stroke data, JSON for meanings/translations.

---

## Building from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+) and npm
- Platform build tools required by Tauri — see the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/) for your OS.

### Steps

```bash
git clone https://github.com/atxxxm/KANJI-MASTER
cd KANJI-MASTER
npm install
```

Run in development mode:

```bash
npm run tauri dev
```

Build a release installer/binary:

```bash
npm run tauri build
```

On first launch, Kanji Master creates its config, database, and data folders automatically.

---

## Where is my data stored?

- **Windows:** `%APPDATA%\atom\kanjimaster\config\`
- **Linux:** `~/.config/atom/kanjimaster/`
- **macOS:** `~/Library/Application Support/atom.kanjimaster/`

To move your setup to another machine, copy `config.toml` and your kanji localization JSON file (e.g. `kanji-localization/en.json`) from that folder.

---

## Adding or fixing a translation

- **Kanji meanings/examples:** use the built-in **Translate Kanji** tool, then click **Save progress** — it writes straight to your localization JSON file.
- **Core kanji data** (readings, JLPT level, stroke count): lives in `core.db` (SQLite) — editable with any SQLite browser.

---

## License

Distributed under the **[GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html)** license. You're free to use, modify, and distribute this software.

---

## Credits

- Font: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP) (Google, OFL license)
- Dictionary data based on **EDICT / JMdict** and **KANJIDIC**
- Stroke-order and handwriting-recognition data from the [KanjiVG](https://kanjivg.tagaini.net/) project
- Built with [Tauri](https://tauri.app/), [React](https://react.dev/), and the wider Rust/npm ecosystems

[github.com/atxxxm/KANJI-MASTER](https://github.com/atxxxm/KANJI-MASTER)
