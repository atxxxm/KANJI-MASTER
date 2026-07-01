import { useState, useEffect, useMemo, useRef } from "react";
import type { KanjiDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useSettings } from "../contexts/SettingsContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";

type JlptFilter = "all" | "N5" | "N4" | "N3" | "N2" | "N1";
const JLPT_LEVELS: JlptFilter[] = ["all", "N5", "N4", "N3", "N2", "N1"];

interface Props {
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
  active?: boolean;
  /** Bumped (to a new timestamp) by App when Ctrl+F focuses this tab. */
  focusSearchAt?: number;
}

export default function KanjiList({ onOpenKanji, onOpenKanjiNewTab, active, focusSearchAt }: Props) {
  const { open } = useContextMenu();
  const { config } = useSettings();
  const showMeaning = config?.show_kanji_meaning ?? false;
  const { kanjiList, loading } = useKanjiData();
  const [search, setSearch]   = useState("");
  const [jlpt, setJlpt]       = useState<JlptFilter>("all");

  const searchRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (focusSearchAt) searchRef.current?.focus();
  }, [focusSearchAt]);

  // Escape clears the search field, but only while this tab is the visible one.
  useEffect(() => {
    if (!active) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") setSearch("");
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [active]);

  const filtered = useMemo(() => {
    const q = search.trim().toLowerCase();
    return kanjiList.filter(k => {
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
  }, [kanjiList, search, jlpt]);

  return (
    <div className="kanji-list-main">
      <div className="kanji-list-toolbar">
        <input
          ref={searchRef}
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
              className={`kanji-card${showMeaning && k.meaning ? " with-meaning" : ""}`}
              onClick={() => onOpenKanji(k)}
              onContextMenu={e => {
                e.preventDefault();
                open(e.clientX, e.clientY, kanjiMenuItems(k, onOpenKanji, onOpenKanjiNewTab));
              }}
            >
              <span className="kanji-char">{k.kanji}</span>
              {showMeaning && k.meaning && (
                <span className="kanji-card-meaning">{k.meaning}</span>
              )}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
