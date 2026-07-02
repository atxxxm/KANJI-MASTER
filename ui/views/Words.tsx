import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto, WordDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useSettings } from "../contexts/SettingsContext";
import { useLocalization } from "../contexts/LocalizationContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";
import "../styles/words.css";

interface Props {
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
  onOpenWord: (wordId: number, wordLabel: string) => void;
  active?: boolean;
}

/// Picks which gloss to show: Russian when the interface language is
/// Russian (with English fallback), English otherwise (Russian fallback).
export function pickGloss(word: WordDto, preferRu: boolean): string {
  if (preferRu) return word.gloss_ru ?? word.gloss_en ?? "";
  return word.gloss_en ?? word.gloss_ru ?? "";
}

/// Renders a Japanese word with each dictionary kanji clickable.
export function ClickableWord({ text, kanjiByChar, onOpenKanji, onOpenKanjiNewTab }: {
  text: string;
  kanjiByChar: Map<string, KanjiDto>;
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
}) {
  const { open } = useContextMenu();
  const { t } = useLocalization();

  return (
    <>
      {[...text].map((ch, i) => {
        const match = kanjiByChar.get(ch);
        return match ? (
          <span
            key={i}
            className="example-kanji-link"
            onClick={e => {
              e.stopPropagation();
              onOpenKanji(match);
            }}
            onContextMenu={e => {
              e.preventDefault();
              e.stopPropagation();
              open(e.clientX, e.clientY, kanjiMenuItems(match, onOpenKanji, onOpenKanjiNewTab, t));
            }}
          >
            {ch}
          </span>
        ) : (
          <span key={i}>{ch}</span>
        );
      })}
    </>
  );
}

export default function Words({ onOpenKanji, onOpenKanjiNewTab, active }: Props) {
  const { kanjiList } = useKanjiData();
  const { config } = useSettings();
  const { t } = useLocalization();

  const [search, setSearch] = useState("");
  const [results, setResults] = useState<WordDto[]>([]);
  const [searched, setSearched] = useState(false);

  const preferRu = config?.interface_language === "Русский";
  const kanjiByChar = useMemo(() => new Map(kanjiList.map(k => [k.kanji, k])), [kanjiList]);

  // Debounced search-as-you-type.
  useEffect(() => {
    const q = search.trim();
    if (!q) {
      setResults([]);
      setSearched(false);
      return;
    }
    const timer = setTimeout(async () => {
      const words = await invoke<WordDto[]>("search_words", { query: q });
      setResults(words);
      setSearched(true);
    }, 200);
    return () => clearTimeout(timer);
  }, [search]);

  // Escape clears the search while this tab is visible.
  useEffect(() => {
    if (!active) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") setSearch("");
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [active]);

  return (
    <div className="words-view">
      <div className="kanji-list-toolbar">
        <input
          className="search-input"
          placeholder={t("words.search_hint")}
          value={search}
          onChange={e => setSearch(e.target.value)}
          autoFocus
        />
        {searched && <span className="result-count">{results.length}</span>}
      </div>

      {!searched ? (
        <div className="view-placeholder">
          <span className="view-placeholder-icon">詞</span>
          <span>{t("words.start_typing")}</span>
        </div>
      ) : results.length === 0 ? (
        <div className="view-placeholder">
          <span>{t("words.no_results")}</span>
        </div>
      ) : (
        <div className="words-list">
          {results.map(w => (
            <div
              key={w.id}
              className="word-row clickable"
              onClick={() => onOpenWord(w.id, w.kanji ?? w.reading)}
            >
              <div className="word-row-main">
                <span className="word-text">
                  <ClickableWord
                    text={w.kanji ?? w.reading}
                    kanjiByChar={kanjiByChar}
                    onOpenKanji={onOpenKanji}
                    onOpenKanjiNewTab={onOpenKanjiNewTab}
                  />
                </span>
                {w.kanji && <span className="word-reading">{w.reading}</span>}
              </div>
              <div className="word-gloss">{pickGloss(w, preferRu)}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
