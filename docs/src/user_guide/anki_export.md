# Anki Export Tool

The **Anki Export** feature allows you to seamlessly transfer the kanji you are studying directly into [Anki](https://apps.ankiweb.net/), the popular spaced repetition flashcard software.

## Step-by-Step Export

The export process is divided into three simple steps:

### Step 1: Select Kanji
Use the search bar and the **JLPT dropdown filter** to find the kanji you want to study. 
Click on a kanji card to select it (it will become highlighted). Click again to deselect. Once you have selected your kanji, click **Next**.

### Step 2: Review Selection
Here you will see a list of all the kanji you've selected. You can review their readings. If you accidentally added a kanji you don't need, click the **❌** button next to it. Click **Next** to proceed.

### Step 3: Export Options
Choose exactly which data fields you want to include in your Anki cards:
*   **Kanji** (The character itself)
*   **Meaning** (Your localized translation)
*   **Onyomi & Kunyomi** (Readings)
*   **JLPT Level**
*   **Examples (HTML)** (Usage examples, formatted with line breaks for Anki)

Click the green **Export to TSV** button and choose where to save the `.txt` file on your computer.

---

## How to Import the File into Anki

Once you have your exported `.txt` file, follow these steps to get it into Anki:

1. Open **Anki** and click on **Import File** at the bottom of the main window.
2. Select the `.txt` file you just exported from Kanji Master.
3. In the Import window, ensure the following settings are correct:
   * **Type:** Basic (or any custom Note Type you prefer).
   * **Deck:** Choose the deck where you want the cards to go.
   * **Field separator:** Make sure it is set to **Tab** (Kanji Master exports as Tab-Separated Values).
   * **Allow HTML in fields:** Make sure this is **Checked** ✅ (This is required for the "Examples" field to display multiple lines correctly).
4. Map the fields from the file (Field 1, Field 2, etc.) to your Anki note's Front and Back fields.
5. Click **Import**.