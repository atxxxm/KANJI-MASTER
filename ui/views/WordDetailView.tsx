import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto, WordDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useLocalization } from "../contexts/LocalizationContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";
import "../styles/word-detail.css";

interface Props {
  wordId: number;
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
}

const isCjk = (ch: string) => {
  const c = ch.charCodeAt(0);
  return c >= 0x4e00 && c <= 0x9faf;
};

export default function WordDetailView({ wordId, onOpenKanji, onOpenKanjiNewTab }: Props) {
  const { kanjiList } = useKanjiData();
  const { open } = useContextMenu();
  const { t } = useLocalization();

  const [word, setWord] = useState<WordDto | null>(null);
  const [loading, setLoading] = useState(true);
  const kanjiByChar = useMemo(() => new Map(kanjiList.map(k => [k.kanji, k])), [kanjiList]);

  useEffect(() => {
    setLoading(true);
    invoke<WordDto | null>("get_word", { id: wordId }).then(w => {
      setWord(w);
      setLoading(false);
    });
  }, [wordId]);

  // Distinct dictionary kanji making up this word, in order of appearance.
  const kanjiChars = useMemo(() => {
    if (!word?.kanji) return [];
    const seen = new Set<string>();
    const out: KanjiDto[] = [];
    for (const ch of word.kanji) {
      if (isCjk(ch) && !seen.has(ch)) {
        seen.add(ch);
        const k = kanjiByChar.get(ch);
        if (k) out.push(k);
      }
    }
    return out;
  }, [word, kanjiByChar]);

  if (loading) return null;
  if (!word) {
    return (
      <div className="word-detail-view">
        <div className="word-detail-inner">
          <div className="detail-not-found">{t("words.no_results")}</div>
        </div>
      </div>
    );
  }

  return (
    <div className="word-detail-view">
      <div className="word-detail-inner">
        <div className="word-detail-head">
          <span className="word-detail-main">{word.kanji ?? word.reading}</span>
          {word.kanji && <span className="word-detail-reading">{word.reading}</span>}
        </div>

        {word.gloss_en && (
          <div className="detail-section">
            <div className="detail-label">English</div>
            <div className="detail-meaning">{word.gloss_en}</div>
          </div>
        )}

        {word.gloss_ru && (
          <div className="detail-section">
            <div className="detail-label">Русский</div>
            <div className="detail-meaning">{word.gloss_ru}</div>
          </div>
        )}

        {kanjiChars.length > 0 && (
          <div className="detail-section">
            <div className="detail-label">{t("current_kanji.components")}</div>
            <div className="word-detail-kanji">
              {kanjiChars.map(k => (
                <button
                  key={k.id}
                  type="button"
                  className="word-detail-kanji-card"
                  onClick={() => onOpenKanji(k)}
                  onContextMenu={e => {
                    e.preventDefault();
                    open(e.clientX, e.clientY, kanjiMenuItems(k, onOpenKanji, onOpenKanjiNewTab, t));
                  }}
                >
                  <span className="word-detail-kanji-char">{k.kanji}</span>
                  {k.meaning && <span className="word-detail-kanji-meaning">{k.meaning}</span>}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
