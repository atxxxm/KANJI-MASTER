# Interface Localization

The **`localization.toml`** file is responsible for **all GUI text labels**: buttons, headers, tooltips, and error messages.
The file structure corresponds to the Rust structures defined in `localization.rs`.

---

## Example Structure (TOML)

```toml
lang = "English"

[local.home]
kanji_search_hint = "Type meanings, reading or kanji..."
kanji_not_found = "No kanji found"

[local.top_bar.kanji]
title = "Kanji"
all = "All"

[local.top_bar.tools]
title = "Tools"
romaji_to_kana = "Romaji to Kana"
translate_kanji = "Translate Kanji"
anki_export = "Export to Anki"
```

---

## Creating a New Translation

1.  Create a copy of the `localization.toml` file.
2.  Translate the field values into the target language.
3.  In the application settings, select the new file for use in the interface.

> ⚠ **Tip:** Ensure the TOML structure and all keys are preserved so the application displays translations correctly.