import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence } from "framer-motion";
import type { KanjiDto } from "../api/types";
import KanjiDetailPanel from "../components/KanjiDetailPanel";
import "../styles/kanji-list.css";
import "../styles/home.css";

const RANDOM_KANJI_KEY = "home_random_kanji_id";

export default function Home() {
  const [allKanji, setAllKanji] = useState<KanjiDto[]>([]);
  const [randomKanji, setRandomKanji] = useState<KanjiDto | null>(null);
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<KanjiDto | null>(null);

  useEffect(() => {
    invoke<KanjiDto[]>("get_kanji_list").then(list => {
      setAllKanji(list);
      if (list.length === 0) return;

      // Picked once per app launch (sessionStorage clears on restart),
      // stable across navigating away from and back to Home.
      const storedId = sessionStorage.getItem(RANDOM_KANJI_KEY);
      let chosen = storedId ? list.find(k => k.id === Number(storedId)) : undefined;
      if (!chosen) {
        chosen = list[Math.floor(Math.random() * list.length)];
        sessionStorage.setItem(RANDOM_KANJI_KEY, String(chosen.id));
      }
      setRandomKanji(chosen);
    });
  }, []);

  const query = search.trim().toLowerCase();
  const hasQuery = query.length > 0;

  const results = useMemo(() => {
    if (!hasQuery) return [];
    return allKanji.filter(k =>
      k.kanji.includes(query) ||
      k.onyomi.toLowerCase().includes(query) ||
      k.kunyomi.toLowerCase().includes(query) ||
      k.onyomi_romaji.toLowerCase().includes(query) ||
      k.kunyomi_romaji.toLowerCase().includes(query) ||
      (k.meaning?.toLowerCase().includes(query) ?? false)
    );
  }, [allKanji, query, hasQuery]);

  return (
    <div className="home-layout">
      <div className="home-view">
        <div className={`home-hero ${hasQuery ? "has-query" : "empty-query"}`}>
          <div className="home-title">KANJI MASTER</div>

          {!hasQuery && (
            <>
              <div className="home-subtitle">Learn, search, and review Japanese kanji</div>

              {randomKanji && (
                <button className="home-random" onClick={() => setSelected(randomKanji)}>
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
                    className={`kanji-card${selected?.id === k.id ? " selected" : ""}`}
                    onClick={() => setSelected(s => s?.id === k.id ? null : k)}
                  >
                    <span className="kanji-char">{k.kanji}</span>
                  </button>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      <AnimatePresence>
        {selected && (
          <KanjiDetailPanel
            kanji={selected}
            onClose={() => setSelected(null)}
            onKanjiClick={setSelected}
          />
        )}
      </AnimatePresence>
    </div>
  );
}
