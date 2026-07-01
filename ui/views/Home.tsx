import { useState, useEffect, useMemo } from "react";
import type { KanjiDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";
import "../styles/home.css";

const RANDOM_KANJI_KEY = "home_random_kanji_id";

interface Props {
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
}

export default function Home({ onOpenKanji, onOpenKanjiNewTab }: Props) {
  const { kanjiList } = useKanjiData();
  const { open } = useContextMenu();
  const [randomKanjiId, setRandomKanjiId] = useState<number | null>(null);
  const [search, setSearch] = useState("");

  useEffect(() => {
    if (kanjiList.length === 0 || randomKanjiId !== null) return;

    // Picked once per app launch (sessionStorage clears on restart),
    // stable across navigating away from and back to Home.
    const storedId = sessionStorage.getItem(RANDOM_KANJI_KEY);
    const id = storedId && kanjiList.some(k => k.id === Number(storedId))
      ? Number(storedId)
      : kanjiList[Math.floor(Math.random() * kanjiList.length)].id;
    sessionStorage.setItem(RANDOM_KANJI_KEY, String(id));
    setRandomKanjiId(id);
  }, [kanjiList, randomKanjiId]);

  // Looked up fresh from kanjiList (rather than cached) so it stays in sync
  // if meanings get reloaded elsewhere (Translate Kanji save, Settings reload).
  const randomKanji = randomKanjiId !== null ? kanjiList.find(k => k.id === randomKanjiId) ?? null : null;

  const query = search.trim().toLowerCase();
  const hasQuery = query.length > 0;

  const results = useMemo(() => {
    if (!hasQuery) return [];
    return kanjiList.filter(k =>
      k.kanji.includes(query) ||
      k.onyomi.toLowerCase().includes(query) ||
      k.kunyomi.toLowerCase().includes(query) ||
      k.onyomi_romaji.toLowerCase().includes(query) ||
      k.kunyomi_romaji.toLowerCase().includes(query) ||
      (k.meaning?.toLowerCase().includes(query) ?? false)
    );
  }, [kanjiList, query, hasQuery]);

  return (
    <div className="home-view">
      <div className={`home-hero ${hasQuery ? "has-query" : "empty-query"}`}>
        <div className="home-title">KANJI MASTER</div>

        {!hasQuery && (
          <>
            <div className="home-subtitle">Learn, search, and review Japanese kanji</div>

            {randomKanji && (
              <button
                className="home-random"
                onClick={() => onOpenKanji(randomKanji)}
                onContextMenu={e => {
                  e.preventDefault();
                  open(e.clientX, e.clientY, kanjiMenuItems(randomKanji, onOpenKanji, onOpenKanjiNewTab));
                }}
              >
                <span className="home-random-label">Kanji of the session</span>
                <span className="home-random-char">{randomKanji.kanji}</span>
                <span className="home-random-reading">
                  {[randomKanji.onyomi, randomKanji.kunyomi].filter(Boolean).join(" · ") || randomKanji.meaning}
                </span>
              </button>
            )}
          </>
        )}

        <div className="home-search-bar" style={!hasQuery ? { marginTop: 6 } : undefined}>
          <span className="home-search-icon">🔍</span>
          <input
            className="home-search-input"
            placeholder="Search kanji, readings, meaning…"
            value={search}
            onChange={e => setSearch(e.target.value)}
            autoFocus
          />
        </div>
      </div>

      {hasQuery && (
        <div className="home-results">
          {results.length === 0 ? (
            <div className="home-results-empty">No kanji found for "{search.trim()}"</div>
          ) : (
            <div className="kanji-grid" style={{ padding: 0 }}>
              {results.map(k => (
                <button
                  key={k.id}
                  className="kanji-card"
                  onClick={() => onOpenKanji(k)}
                  onContextMenu={e => {
                    e.preventDefault();
                    open(e.clientX, e.clientY, kanjiMenuItems(k, onOpenKanji, onOpenKanjiNewTab));
                  }}
                >
                  <span className="kanji-char">{k.kanji}</span>
                </button>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
