import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence } from "framer-motion";
import type { KanjiDto } from "../api/types";
import KanjiDetailPanel from "../components/KanjiDetailPanel";
import "../styles/kanji-list.css";

type JlptFilter = "all" | "N5" | "N4" | "N3" | "N2" | "N1";
const JLPT_LEVELS: JlptFilter[] = ["all", "N5", "N4", "N3", "N2", "N1"];

export default function KanjiList() {
  const [kanji, setKanji]     = useState<KanjiDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch]   = useState("");
  const [jlpt, setJlpt]       = useState<JlptFilter>("all");
  const [selected, setSelected] = useState<KanjiDto | null>(null);

  useEffect(() => {
    invoke<KanjiDto[]>("get_kanji_list").then(data => {
      setKanji(data);
      setLoading(false);
    });
  }, []);

  const filtered = useMemo(() => {
    const q = search.trim().toLowerCase();
    return kanji.filter(k => {
      if (jlpt !== "all" && k.jlpt !== jlpt) return false;
      if (!q) return true;
      return (
        k.kanji.includes(q) ||
        k.onyomi.toLowerCase().includes(q) ||
        k.kunyomi.toLowerCase().includes(q) ||
        k.onyomi_romaji.toLowerCase().includes(q) ||
        k.kunyomi_romaji.toLowerCase().includes(q) ||
        (k.meaning?.toLowerCase().includes(q) ?? false)
      );
    });
  }, [kanji, search, jlpt]);

  const handleCardClick = (k: KanjiDto) => {
    setSelected(prev => prev?.id === k.id ? null : k);
  };

  return (
    <div className="kanji-list-layout">
      <div className="kanji-list-main">
        <div className="kanji-list-toolbar">
          <input
            className="search-input"
            placeholder="Search kanji, readings, meaning…"
            value={search}
            onChange={e => setSearch(e.target.value)}
            autoFocus
          />
          <div className="jlpt-filters">
            {JLPT_LEVELS.map(level => (
              <button
                key={level}
                className={`jlpt-btn f-${level}${jlpt === level ? " active" : ""}`}
                onClick={() => setJlpt(level)}
              >
                {level === "all" ? "All" : level}
              </button>
            ))}
          </div>
          <span className="result-count">{filtered.length}</span>
        </div>

        {loading ? (
          <div className="view-placeholder">
            <span className="view-placeholder-icon">漢</span>
            <span>Loading kanji…</span>
          </div>
        ) : (
          <div className="kanji-grid">
            {filtered.map(k => (
              <button
                key={k.id}
                className={`kanji-card${selected?.id === k.id ? " selected" : ""}`}
                onClick={() => handleCardClick(k)}
              >
                <span className="kanji-char">{k.kanji}</span>
              </button>
            ))}
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
