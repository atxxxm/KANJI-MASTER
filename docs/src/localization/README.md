# Localization

**Kanji Master** fully supports localization through external JSON files.
This allows translating the interface and content without needing to recompile the program.

---

## File Locations

Upon first launch, localization files are automatically extracted to the **system configuration folder**:

*   **Windows:**
    `C:\Users\<Username>\AppData\Roaming\KanjiMaster\localization\`
*   **Linux:**
    `/home/<username>/.config/kanjimaster/localization/`
*   **macOS:**
    `/Users/<Username>/Library/Application Support/KanjiMaster/localization/`

> You can open the current folder with localization files directly from the application settings by clicking the **📂** button.

---

## Key Files

The system uses two key JSON files:

1.  **Interface:** `localization.json` – contains translations for user interface elements.
2.  **Data:** `kanji-localization.json` – contains translations for kanji meanings and usage examples.

> By editing these files, you can add your own translations or modify existing ones without changing the application's code.