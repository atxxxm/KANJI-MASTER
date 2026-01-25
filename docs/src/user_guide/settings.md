# Settings

The **Settings** menu allows you to adapt **Kanji Master** to your preferences: from font size to managing localization files.

> To open the settings panel, click the gear icon (**⚙**) on the right side of the top navigation bar.

---

## Files and Data

This section manages paths to external resource files.
It is useful if you want to use your own translations or switch between different language packs.

*   **Localization File** – path to the main user interface translation file (`localization.json`).
*   **Kanji Data File** – path to the file containing translations for kanji meanings and examples (`kanji-localization.json`).

**Managing files:**

*   Click the **📂** button next to a path to select a different `.json` file via the system dialog.
*   The application will automatically copy the selected file to the configuration folder and start using it.

---

## Appearance

Customize the visual presentation of the application for comfortable reading:

*   **Interface Font Size** – size of the main interface text (buttons, menus, headers).
    *Range:* 12px — 32px

*   **Kanji Font Size** – size of kanji characters in lists and on cards.
    *Range:* 20px — 120px

*   **Animation Speed** – speed at which strokes are drawn during kanji writing animation.
    *Scale:* Logarithmic (from 0.1x to 5.0x). Higher value = slower animation.

---

## Behavior

Options affecting how you interact with the application:

### Focus on Search

If enabled, when the application starts or you switch to the main screen, the **cursor is automatically placed in the search field**.
*Recommended for:* quick searching for kanji or words.

### Show Kanji Meaning

Toggle for displaying kanji meanings in lists and search results:

*   **Enabled:** a brief meaning is displayed under each kanji.
*   **Disabled:** only the kanji character itself is shown – useful for self-testing and memory training.

---

## Tools

Additional utilities for working with configuration:

*   **Create Default Localization File** – creates a template localization file (`default_localization.json`).
    Useful for translators or for those who want to create a localization from scratch, having the full structure of keys as a reference.

> **Note:** All changes are automatically saved to the configuration file when you close the settings window or exit the application.