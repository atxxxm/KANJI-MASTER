import { useState, useEffect, useMemo, useRef } from "react";
import type { KanjiDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useSettings } from "../contexts/SettingsContext";
import { useLocalization } from "../contexts/LocalizationContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";

type JlptFilter = "all" | "N5" | "N4" | "N3" | "N2" | "N1";
const JLPT_LEVELS: JlptFilter[] = ["all", "N5", "N4", "N3", "N2", "N1"];

// KANJIDIC grade codes: 1-6 are the elementary-school years, 8 is the rest
// of the jōyō (general-use) set, 9 is jinmeiyō (name-only) kanji — there is
// no grade 7 in the data.
type GradeFilter = "all" | "1" | "2" | "3" | "4" | "5" | "6" | "8" | "9";
const GRADE_LEVELS: GradeFilter[] = ["all", "1", "2", "3", "4", "5", "6", "8", "9"];

type StrokesFilter = "all" | "1-4" | "5-8" | "9-12" | "13-16" | "17+";
const STROKE_RANGES: { key: StrokesFilter; min: number; max: number }[] = [
  { key: "1-4",   min: 1,  max: 4 },
  { key: "5-8",   min: 5,  max: 8 },
  { key: "9-12",  min: 9,  max: 12 },
  { key: "13-16", min: 13, max: 16 },
  { key: "17+",   min: 17, max: Infinity },
];

// Frequency is a near-unique rank (1 = most common), so it's bucketed into
// "top N" ranges rather than filtered by exact value. Kanji with no rank
// (frequency === "") fall outside the top ~2500 and are treated as "rare".
type FreqFilter = "all" | "top500" | "top1000" | "top2000" | "rare";
const FREQ_CAPS: Record<Exclude<FreqFilter, "all" | "rare">, number> = {
  top500: 500,
  top1000: 1000,
  top2000: 2000,
};

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
  const { t } = useLocalization();
  const showMeaning = config?.show_kanji_meaning ?? false;
  const { kanjiList, loading } = useKanjiData();
  const [search, setSearch]   = useState("");
  const [jlpt, setJlpt]       = useState<JlptFilter>("all");
  const [grade, setGrade]     = useState<GradeFilter>("all");
  const [strokes, setStrokes] = useState<StrokesFilter>("all");
  const [freq, setFreq]       = useState<FreqFilter>("all");

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
    const strokeRange = STROKE_RANGES.find(r => r.key === strokes);
    return kanjiList.filter(k => {
      if (jlpt !== "all" && k.jlpt !== jlpt) return false;
      if (grade !== "all" && k.grade !== grade) return false;
      if (strokeRange && (k.strokes < strokeRange.min || k.strokes > strokeRange.max)) return false;
      if (freq !== "all") {
        const rank = k.frequency ? Number(k.frequency) : null;
        if (freq === "rare") {
          if (rank !== null) return false;
        } else {
          if (rank === null || rank > FREQ_CAPS[freq]) return false;
        }
      }
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
  }, [kanjiList, search, jlpt, grade, strokes, freq]);

  return (
    <div className="kanji-list-main">
      <div className="kanji-list-toolbar">
        <input
          ref={searchRef}
          className="search-input"
          placeholder={t("kanji_list.search_hint")}
          aria-label={t("kanji_list.search_hint")}
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
              {level === "all" ? t("kanji_list.all_filter") : level}
            </button>
          ))}
        </div>
        <span className="result-count">{filtered.length}</span>
      </div>

      <div className="kanji-list-filters-row">
        <div className="filter-group">
          <span className="filter-group-label">{t("kanji_list.grade_label")}</span>
          {GRADE_LEVELS.map(g => (
            <button
              key={g}
              className={`filter-btn${grade === g ? " active" : ""}`}
              onClick={() => setGrade(g)}
            >
              {g === "all" ? t("kanji_list.all_filter") : g === "8" ? t("kanji_list.grade_joyo") : g === "9" ? t("kanji_list.grade_names") : g}
            </button>
          ))}
        </div>

        <div className="filter-group">
          <span className="filter-group-label">{t("kanji_list.strokes_label")}</span>
          <button
            className={`filter-btn${strokes === "all" ? " active" : ""}`}
            onClick={() => setStrokes("all")}
          >
            {t("kanji_list.all_filter")}
          </button>
          {STROKE_RANGES.map(r => (
            <button
              key={r.key}
              className={`filter-btn${strokes === r.key ? " active" : ""}`}
              onClick={() => setStrokes(r.key)}
            >
              {r.key}
            </button>
          ))}
        </div>

        <div className="filter-group">
          <span className="filter-group-label">{t("kanji_list.frequency_label")}</span>
          <button
            className={`filter-btn${freq === "all" ? " active" : ""}`}
            onClick={() => setFreq("all")}
          >
            {t("kanji_list.all_filter")}
          </button>
          <button className={`filter-btn${freq === "top500" ? " active" : ""}`} onClick={() => setFreq("top500")}>
            {t("kanji_list.freq_top500")}
          </button>
          <button className={`filter-btn${freq === "top1000" ? " active" : ""}`} onClick={() => setFreq("top1000")}>
            {t("kanji_list.freq_top1000")}
          </button>
          <button className={`filter-btn${freq === "top2000" ? " active" : ""}`} onClick={() => setFreq("top2000")}>
            {t("kanji_list.freq_top2000")}
          </button>
          <button className={`filter-btn${freq === "rare" ? " active" : ""}`} onClick={() => setFreq("rare")}>
            {t("kanji_list.freq_rare")}
          </button>
        </div>
      </div>

      {loading ? (
        <div className="view-placeholder">
          <span className="view-placeholder-icon">漢</span>
          <span>{t("kanji_list.loading")}</span>
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
                open(e.clientX, e.clientY, kanjiMenuItems(k, onOpenKanji, onOpenKanjiNewTab, t));
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
