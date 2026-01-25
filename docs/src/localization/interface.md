# Interface Localization

The **`localization.json`** file is responsible for **all GUI text labels**: buttons, headers, tooltips, and error messages.

The file structure corresponds to the Rust structures defined in `localization.rs`.

---

## Example Structure (JSON)

```json
{
  "lang": "English",
  "local": {
    "home": {
      "kanji_search_hint": "Search Kanji, Kana, Meaning...",
      "kanji_not_found": "No Kanji found matching your criteria"
    },
    "top_bar": {
      "kanji": { "title": "Kanji", "all": "All" },
      "cards": { "title": "Cards", "all": "All" },
      "tools": { "title": "Tools" }
    },
    "settings": {
      "title": "Settings",
      "appearance": "Appearance"
    },
    "screens": {},
    "kana": {}
  }
}
```

---

## Creating a New Translation

1.  Create a copy of the `localization.json` file.
2.  Translate the field values into the target language.
3.  In the application settings, select the new file for use in the interface.

> ⚠ **Tip:** Ensure the JSON structure and all keys are preserved so the application displays translations correctly.