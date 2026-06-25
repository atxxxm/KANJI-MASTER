# FAQ and Troubleshooting

---

## Where is my data stored?

All configuration, database, and localization files are stored in the system config directory:

- **Windows:** `C:\Users\<Username>\AppData\Roaming\atom\kanjimaster\config\`
- **Linux:** `/home/<username>/.config/atom/kanjimaster/`
- **macOS:** `/Users/<Username>/Library/Application Support/atom.kanjimaster/`

To transfer your setup to another computer, copy:
- `config.toml` — your settings and file paths
- your kanji localization file (e.g. `kanji-localization/en.json`) if you have custom translations

---

## Kanji meanings are not showing

On first launch a setup screen appears asking you to select a localization file. If you dismissed it with **Skip**, meanings will be empty.

To fix: open **Settings (⚙) → Files & Data**, click **📂** next to "Kanji Localization", and pick `en.json` (or another language file) from the `kanji-localization/` folder in your config directory.

---

## The application is slow or animations stutter

Make sure you are running in **Release mode**. Debug builds include extra checks that significantly slow down the UI.

```bash
cargo run --release
```

---

## How do I reset all settings?

1. Close the application.
2. Navigate to the config directory (paths listed above).
3. Delete the entire folder.
4. On the next launch, all files will be recreated with defaults.

---

## How do I add a translation for my language?

**UI language:** copy any `.toml` file from the `localization/` folder, translate the values, then load it in **Settings → Language (TOML)**.

**Kanji meanings:** copy `kanji-localization/en.json`, translate the `meaning` and `translate_examples` fields, then load it in **Settings → Kanji Localization (JSON)**. The built-in **Translate Kanji Tool** (Tools menu) makes this easier with a guided UI.

---

## I found an error in a translation

- For **kanji meanings or examples**: use the **Translate Kanji Tool** to correct the entry, then click **Save Progress**.
- For **UI strings**: open the `.toml` localization file in a text editor, fix the value, and reload it in Settings.
- For **core kanji data** (wrong reading, JLPT level, stroke count): this lives in `core.db`. You can edit it with any SQLite browser (e.g. DB Browser for SQLite).

---

## The draw & search doesn't find the right kanji

A few tips for better recognition accuracy:

- **Match the stroke count exactly.** The algorithm filters candidates by stroke count (±2). If a kanji has 5 strokes, draw 5 strokes.
- **Use the Undo button** (⬅) to remove a wrong stroke and redraw it.
- **Center and proportion your drawing** — relative sizes of radicals matter.
- Stroke order affects accuracy but is not strictly required; the algorithm uses order-independent matching internally.
