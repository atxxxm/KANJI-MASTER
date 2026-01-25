# Tools

The **`Tools`** menu provides auxiliary utilities for working with text and localization.

## Romaji to Kana Converter

Allows you to **convert text typed in Latin letters (Romaji) into Japanese syllabaries** in real time.

*   **Input:** Type text, for example, `konnichiwa`.
*   **Output:** Get `こんにちは` (or katakana, if the toggle is set accordingly).
*   **Copying:** The **Copy** button copies the result to the clipboard.

The conversion logic accounts for:

*   Vowels and consonants.
*   Double consonants via `っ` / `ッ`.
*   Special combinations (yo-on), such as `kya`, `shu`.
*   Punctuation: `.`, `,`, `-`.

## Translate Kanji Tool

A built-in tool for **creating and editing kanji localization**.

1.  **Search**
    Enter an **ID** or the character itself to navigate to it, or use the **Previous / Next** buttons.

2.  **Editing**

    *   **Meaning** – a field for translating the kanji's meaning.
    *   **Examples** – fields for translating usage examples.

3.  **Saving**
    Click **Save Progress** to write the changes to a local JSON file.

> ⚠ **Important:** changes are saved only to your local file within the configuration folder.