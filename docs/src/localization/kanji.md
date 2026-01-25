# Kanji Localization

The **`kanji-localization.json`** file (or any other file selected in the settings) contains translations for **kanji meanings** and **usage examples**.

---

## File Structure

The root object contains an `entries` field where:

*   **Key** – the kanji character itself (e.g., `"日"`).
*   **Value** – an object with the translation and examples.

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
```

---

### 🔹 Field Descriptions

*   **meaning** – the main meaning of the kanji (string).
*   **translate_examples** – an array of strings containing translations for the usage examples.

    *   The order of elements must match the order of the original Japanese examples from the `core.db` database.

> 💡 Tip: For convenient editing, use the built-in **Translate Kanji Tool** within the application.
> It automatically synchronizes example indexes and keeps the file structure correct.