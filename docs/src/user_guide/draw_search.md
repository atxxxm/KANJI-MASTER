# Draw & Search

Encountered a kanji you don't know how to read? Draw it with your mouse and Kanji Master will find the closest matches in the database.

---

## How to Use

1. Open the **Tools** menu and select **Draw & Search**.
2. Draw the kanji on the large canvas using your mouse or stylus. Each continuous drag is one stroke.
3. Use **Undo (⬅)** to remove the last stroke if you made a mistake.
4. Use **Clear (🗑)** to wipe the canvas and start over.
5. Click **Search (🔍)**.

The best matches appear in the panel on the right (or below the canvas on narrow windows). Click any result card to open its detail page. Middle-click to open in a background tab.

---

## Tips for Better Results

**Match the stroke count.**
The algorithm filters candidates by stroke count with a tolerance of ±2. This is the single most important factor. If a kanji has 5 strokes, draw 5 — not 4, not 6.

**Use Undo to fix mistakes.**
It's better to undo and redraw a bad stroke than to leave it in. A wrong stroke shifts the score significantly.

**Maintain proportions.**
Try to center the drawing on the canvas and keep radicals roughly the right size relative to each other.

**Stroke order is helpful but not required.**
The recognition engine uses order-independent matching internally (each drawn stroke is paired with the closest matching template stroke), so you won't be penalized heavily for slightly different ordering.

---

## How the Recognition Algorithm Works

The engine runs in two stages:

**1. Normalization**
Each stroke is resampled to 32 uniformly-spaced points by arc length (equal spacing by distance, not by time). All strokes are then scaled into a unit bounding box to remove position and size differences.

**2. Similarity Scoring**
For each candidate kanji (filtered by stroke count ±2):
- Every drawn stroke is matched to the closest unmatched template stroke using **DTW (Dynamic Time Warping)**. DTW finds the optimal non-linear alignment between two point sequences, making the comparison robust to variations in drawing speed.
- The total score is the sum of DTW distances across matched pairs, plus a penalty for any unmatched strokes.
- Candidates are ranked by score — lower is better.

The top 20 matches are displayed.
