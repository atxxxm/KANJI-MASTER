# Settings

The **Settings** panel allows you to customize Kanji Master's appearance, behavior, and file paths. 
> To open it, click the gear icon (**⚙**) on the far right of the top navigation bar.

---

## Files & Data

This section manages paths to external resource files. It is useful if you want to use custom community translations or switch between different language packs.

*   **Language (TOML)** – Path to the UI translation file (`localization.toml`).
*   **Kanji Localization (JSON)** – Path to the kanji meanings and examples file (`kanji-localization.json`).

**Managing files:**
Click the **📂 Select** button next to a path to choose a different file via your system's file explorer. The application will instantly load the new file.

---

## Appearance

Customize the visual presentation of the application to suit your monitor size and reading preferences:

*   **Interface font size** – Adjusts the size of buttons, menus, and standard text. *(Range: 12px — 32px)*
*   **Kanji font size** – Adjusts the size of kanji characters displayed inside lists and cards. *(Range: 20px — 120px)*
*   **Kanji animation speed** – Controls how fast the strokes are drawn in the Kanji Detail tab. Moves on a logarithmic scale (from 0.1x to 5.0x).

---

## Behavior

*   **Focus on search when opening**
    If enabled, whenever you open a new Home tab, your cursor will be automatically placed in the search box. Highly recommended for keyboard-heavy users.
*   **Show kanji meaning**
    Toggles the display of kanji meanings underneath the characters in the search results and kanji lists. Turn this **off** if you want to use the lists for strict memory recall testing!

---

## Tools

*   **Create default localization file**
    Clicking this button opens a save dialog to generate a fresh, default `localization.toml` template. This is incredibly useful for translators who want to create a new UI language pack from scratch without guessing the required TOML keys.

> **Note:** All changes made in the settings panel are automatically saved to your `config.toml` file the moment you close the window.