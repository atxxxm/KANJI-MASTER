# Kanji Localization (JSON)

The **`kanji-localization.json`** file contains your personal translations for **kanji meanings** and **usage examples**. 

> 🛠 **Highly Recommended:** Instead of editing this raw JSON file manually, use the built-in **Translate Kanji Tool** (found in the Tools menu). The tool provides a clean UI for translating and automatically saves the JSON file with the correct formatting and IDs!

---

## File Structure

If you prefer to edit the file manually or write a script to generate it, the root object contains an `entries` dictionary where:
*   **Key** – the kanji character itself (e.g., `"日"`).
*   **Value** – an object with the translation and an array of examples.

Example structure:

```json
{
  "last_id": 100,
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

Field Descriptions

    last_id – Keeps track of the last kanji you translated using the built-in UI tool.

    meaning – The main meaning of the kanji (String).

    translate_examples – An array of strings containing translations for the usage examples. The order of elements must exactly match the order of the original Japanese examples in the core.db database.