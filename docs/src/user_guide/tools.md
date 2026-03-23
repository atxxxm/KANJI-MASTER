# Other Tools

The **`Tools`** menu provides utility features for text manipulation and localization.

## 1. Romaji to Kana Converter & Kanji Assistant

This tool allows you to convert text typed in Latin letters (Romaji) into Japanese syllabaries in real time.

*   **Input Mode:** Type text in the large editor (e.g., `watashi wa mizu wo nomimasu`).
*   **Output Mode:** Toggle between Hiragana and Katakana output.
*   **Copying:** Use the **Copy Result** button to copy the converted text to your clipboard.

### 💡 The Kanji Assistant
The true power of this tool lies in the **Kanji Assistant panel** on the right side. 

As you type kana, the Assistant continuously analyzes the last word you typed. It searches the database and instantly suggests kanji that match that reading.
*   Typing `みず` will instantly show a card for **水**.
*   Clicking the **水** card in the Assistant will automatically replace `みず` with `水` in your text editor.
*   Cards are smartly sorted: exact matches appear first, followed by JLPT N5-N1 order.

This turns the converter into an interactive learning tool, helping you visually memorize kanji while typing!

---

## 2. Translate Kanji Tool

A built-in editor designed for translating kanji meanings and examples into your native language. 
*(Note: these changes are saved to your local `kanji-localization.json` file).*

1.  **Search & Navigation:**
    *   Type a kanji, romaji reading, or ID into the search box.
    *   A dropdown will appear. Click on a result to jump directly to that kanji.
    *   Alternatively, use the **Previous / Next** buttons to browse sequentially.
2.  **Editing:**
    *   **Meaning:** Type the primary translation of the kanji.
    *   **Examples:** Translate the Japanese vocabulary words into your language. The original Japanese word is shown above the input box for reference.
3.  **Saving:**
    Click the **Save Progress** button at the top right to write all changes to disk.