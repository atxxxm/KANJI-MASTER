# Localization

**Kanji Master** supports two independent layers of localization, both driven by external files — no recompilation needed.

---

## Two Localization Layers

| Layer | File | Controls |
|---|---|---|
| UI | `localization.toml` | All interface text: buttons, labels, tooltips |
| Kanji data | `en.json` / `ru.json` / custom | Kanji meanings and example translations |

These are separate systems. You can use a Russian UI with an English kanji file, or any other combination.

---

## File Locations

On first launch, the bundled files are automatically extracted to the system config directory:

- **Windows:** `C:\Users\<Username>\AppData\Roaming\atom\kanjimaster\config\`
- **Linux:** `/home/<username>/.config/atom/kanjimaster/`
- **macOS:** `/Users/<Username>/Library/Application Support/atom.kanjimaster/`

Bundled files:
```
config/
  localization/
    localization.toml       ← English UI (default)
    localization_ru.toml    ← Russian UI
    localization_de.toml    ← German UI
    ... (other languages)
  kanji-localization/
    en.json                 ← English kanji meanings
    ru.json                 ← Russian kanji meanings
```

---

## Switching Languages

Open **Settings (⚙)** → **Files & Data** and use the **📂** button next to each path to pick a different file. The application reloads the file instantly — no restart needed.

---

## Creating Your Own Translation

The design is intentionally open: if there is no translation for your language, you can create one yourself.

**For UI localization:** copy `localization.toml`, translate the values, and load your file in Settings. See [Interface (TOML)](./interface.md) for the full key reference.

**For kanji meanings:** copy `en.json`, translate the meanings and examples, and load your file in Settings. See [Kanji Data (JSON)](./kanji.md) for the file format.

> Your custom files are never overwritten by application updates, since `initialize_app_data` only extracts files that don't already exist.
