# Kanji Localization (JSON)

Kanji meanings and example translations are stored in a JSON file you choose. The app ships with `en.json` (English) and `ru.json` (Russian). You can create a file for any language by copying one and translating it.

> 🛠 **Recommended:** Use the built-in **Translate Kanji Tool** (Tools menu) instead of editing the JSON directly. It provides a clean UI and saves the file automatically with correct formatting.

---

## First Launch

If no kanji localization file is found, a setup screen appears automatically:

- **Use English (built-in)** — loads the bundled `en.json` immediately.
- **Browse for JSON file** — opens a file picker so you can load a custom file.
- **Skip** — dismisses the screen; meanings will not be shown until a file is loaded.

You can change the file at any time in **Settings (⚙) → Files & Data**.

---

## File Format

The root object has two fields:

```json
{
  "last_id": 42,
  "entries": {
    "日": {
      "meaning": "Day, sun, Japan",
      "translate_examples": [
        "Sunday",
        "Japan",
        "Holiday"
      ]
    },
    "月": {
      "meaning": "Month, moon",
      "translate_examples": [
        "Monday",
        "Next month"
      ]
    }
  }
}
```

### Field Reference

| Field | Type | Description |
|---|---|---|
| `last_id` | `i32` | ID of the last kanji saved by the Translate tool. Used to resume where you left off. |
| `entries` | `object` | Keys are kanji characters (`"日"`), values are translation objects. |
| `meaning` | `string` | Primary meaning of the kanji. |
| `translate_examples` | `string[]` | Translations of the usage examples. Must be in the **same order** as the examples in `core.db`. |

> ⚠️ The order of `translate_examples` matters. Each element corresponds to the example at the same index in the database. Using the Translate Kanji Tool handles this automatically.

---

## Creating a New Language File

1. Copy `en.json` from the config directory.
2. Translate each `"meaning"` value and the strings in `"translate_examples"`.
3. Keep all kanji keys (`"日"`, `"月"`, etc.) unchanged.
4. Load your file in **Settings → Kanji Localization (JSON)**.

The application will copy your file into the config directory and remember the path.
