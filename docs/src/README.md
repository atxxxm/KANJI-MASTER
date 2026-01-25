# Introduction to Kanji Master

**Kanji Master** is a cross-platform desktop application built in **Rust**, designed for learning Japanese. The main focus is on studying **kanji**: their readings, stroke order, and usage in vocabulary.

The application uses:

*   **egui / eframe** for the graphical interface
*   **rusqlite** for working with the kanji database

## Key Features

*   **Kanji Database**
    Search for characters by shape, meaning, readings (onyomi/kunyomi), and JLPT levels.

*   **Stroke Order Animation**
    Visualizes the correct stroke order for each kanji.

*   **Flashcard System**
    Spaced repetition system (SRS) for knowledge review. Ability to create custom decks.

*   **Kana Reference**
    Complete tables for Hiragana and Katakana.

*   **Tools**
    Romaji to Kana converter and a built-in translation editor.

*   **Localization**
    Full support for switching the interface language and kanji meanings via JSON files.

## Tech Stack

*   **Programming Language:** Rust
*   **GUI:** [eframe / egui](https://github.com/emilk/egui)
*   **Database:** SQLite
*   **Configuration:** TOML
*   **Localization:** JSON