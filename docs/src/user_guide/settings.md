# Settings

Open the Settings panel by clicking the **⚙** icon on the right side of the top bar.

> All changes are saved to `config.toml` automatically when you close the window.

---

## Files & Data

Paths to external resource files. Change these to switch languages or use custom translations.

| Setting | File type | Description |
|---|---|---|
| Language (TOML) | `.toml` | UI localization — buttons, labels, menus |
| Kanji Localization (JSON) | `.json` | Kanji meanings and example translations |

Click the **📂** button next to a path to open a file picker. The selected file is copied into the config directory and loaded immediately — no restart needed.

---

## Appearance

**Theme**
Toggle between **☀ Light** and **🌙 Dark** mode. You can also switch quickly using the theme button in the top bar without opening Settings.

**Interface font size**
Adjusts the size of all standard text: buttons, labels, menus. Range: 12–32 px.

**Kanji font size**
Adjusts the size of kanji characters in cards and lists. Range: 20–120 px.

**Kanji animation speed**
Controls how fast strokes are drawn in the Kanji Detail tab. Uses a logarithmic scale (0.1× to 5.0×).

---

## Behavior

**Focus on search when opening**
When enabled, the cursor is automatically placed in the search field every time you open a new Home tab. Useful for keyboard-heavy workflows.

**Show kanji meaning**
Shows the meaning text underneath kanji characters in the search results and kanji list cards. Disable this if you want to use the lists for self-testing.

---

## Tools

**Create default localization file**
Opens a save dialog to generate a fresh `localization.toml` template with all required keys and English default values. Use this as a starting point when creating a UI translation for a new language.
