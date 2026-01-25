# FAQ and Troubleshooting

This section contains answers to **frequently asked questions** and guidance for resolving potential technical issues.

---

## Where is my data stored?

The application stores **configuration, the database, and user-created decks** in the user's system directory:

*   **Windows:**
    `C:\Users\<Username>\AppData\Roaming\KanjiMaster\`
*   **Linux:**
    `/home/<username>/.config/kanjimaster/`
*   **macOS:**
    `/Users/<Username>/Library/Application Support/KanjiMaster/`

> To transfer your progress to another computer:
>
> 1.  Copy the **`config.toml`** file (contains your decks).
> 2.  Copy the **`kanji-localization`** folder, if you have edited translations.

---

## The application is "laggy" or animations are stuttering

Make sure you are **running the application in Release mode**.

In Debug mode, Rust performs numerous checks that can slow down the graphical interface and data processing.

To run in Release mode, use the command:

```bash
cargo run --release
```

---

## How to reset all settings?

If the application fails to start or you wish to return to the **default settings**:

1.  Close the application.
2.  Navigate to the data folder (paths are listed above).
3.  Delete the entire **KanjiMaster** folder.
4.  On the next launch, the program will recreate all files with default settings.

---

## I found an error in a translation or a kanji

*   To correct **localized translations**, use the built-in **Translate Kanji Tool** or edit the JSON file manually.
*   If the error is in the **core kanji data** (e.g., an incorrect reading or JLPT level), you will need to modify the **core.db** file using an SQLite browser, as this data is part of the application's core.