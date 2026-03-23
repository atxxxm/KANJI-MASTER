# Introduction to Kanji Master

**Kanji Master** is a cross-platform desktop application built in **Rust**, designed for learning Japanese. The main focus is on studying **kanji**: their readings, stroke order, and usage in vocabulary.

The application uses:
*   **egui / eframe** for the graphical interface
*   **rusqlite** for working with the kanji database

## Key Features

*   **Kanji Database & Search**
    Search for characters by shape, meaning, readings (onyomi/kunyomi), and JLPT levels.
*   **Draw & Search**
    Draw kanji directly on the screen using your mouse or stylus to find characters you don't know how to read.
*   **Stroke Order Animation**
    Visualizes the correct stroke order for each kanji.
*   **Kana Reference & Romaji Converter**
    Complete tables for Hiragana and Katakana, plus a smart Romaji to Kana converter with a built-in Kanji Assistant.
*   **Anki Export**
    Select kanji, review them, and export them directly to a TSV file ready to be imported into Anki.
*   **Localization & Custom Translations**
    Full support for switching the interface language (TOML) and editing kanji meanings (JSON) via a built-in Translation Tool.

## Tech Stack
*   **Language:** Rust
*   **GUI:** [eframe / egui](https://github.com/emilk/egui)
*   **Database:** SQLite
*   **Configuration & UI Localization:** TOML
*   **Kanji Data Translation:** JSON