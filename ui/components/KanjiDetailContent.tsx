import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import AnimatedKanji from "./AnimatedKanji";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useLocalization } from "../contexts/LocalizationContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";

interface Props {
  kanji: KanjiDto;
  onKanjiClick: (k: KanjiDto) => void;
  onKanjiClickNewTab: (k: KanjiDto) => void;
}

// CJK Unified Ideographs range
const isCjk = (ch: string) => {
  const c = ch.charCodeAt(0);
  return c >= 0x4e00 && c <= 0x9faf;
};

function ExampleText({ text, onKanjiClick, onKanjiClickNewTab }: {
  text: string;
  onKanjiClick: (k: KanjiDto) => void;
  onKanjiClickNewTab: (k: KanjiDto) => void;
}) {
  const { open } = useContextMenu();
  const { t } = useLocalization();

  const resolve = async (ch: string): Promise<KanjiDto | null> =>
    invoke<KanjiDto | null>("get_kanji_by_char", { ch });

  const handleClick = async (e: React.MouseEvent, ch: string) => {
    e.stopPropagation();
    const k = await resolve(ch);
    if (k) onKanjiClick(k);
  };

  const handleContextMenu = async (e: React.MouseEvent, ch: string) => {
    e.preventDefault();
    e.stopPropagation();
    const k = await resolve(ch);
    if (k) open(e.clientX, e.clientY, kanjiMenuItems(k, onKanjiClick, onKanjiClickNewTab, t));
  };

  return (
    <>
      {[...text].map((ch, i) =>
        isCjk(ch) ? (
          <span
            key={i}
            className="example-kanji-link"
            onClick={e => handleClick(e, ch)}
            onContextMenu={e => handleContextMenu(e, ch)}
          >
            {ch}
          </span>
        ) : (
          <span key={i}>{ch}</span>
        )
      )}
    </>
  );
}

// Radical/component breakdown (e.g. 語 -> 言 + 吾), parsed on the backend
// from the kanji's own KanjiVG SVG. Components that also happen to be one
// of our 2209 dictionary kanji are clickable; the rest (many radicals
// aren't standalone jōyō kanji) render as plain, non-interactive glyphs.
function ComponentsRow({ kanji, kanjiByChar, onKanjiClick, onKanjiClickNewTab }: {
  kanji: KanjiDto;
  kanjiByChar: Map<string, KanjiDto>;
  onKanjiClick: (k: KanjiDto) => void;
  onKanjiClickNewTab: (k: KanjiDto) => void;
}) {
  const { open } = useContextMenu();
  const { t } = useLocalization();
  const [components, setComponents] = useState<string[]>([]);

  useEffect(() => {
    let cancelled = false;
    invoke<string[]>("get_kanji_components", { kanjiId: kanji.id }).then(cs => {
      if (!cancelled) setComponents(cs);
    });
    return () => {
      cancelled = true;
    };
  }, [kanji.id]);

  if (components.length === 0) return null;

  return (
    <div className="detail-section">
      <div className="detail-label">{t("current_kanji.components")}</div>
      <div className="detail-components">
        {components.map((ch, i) => {
          const match = kanjiByChar.get(ch);
          return (
            <button
              key={i}
              type="button"
              className={`detail-component${match ? " clickable" : ""}`}
              disabled={!match}
              onClick={() => match && onKanjiClick(match)}
              onContextMenu={e => {
                if (!match) return;
                e.preventDefault();
                open(e.clientX, e.clientY, kanjiMenuItems(match, onKanjiClick, onKanjiClickNewTab, t));
              }}
            >
              {ch}
            </button>
          );
        })}
      </div>
    </div>
  );
}

export default function KanjiDetailContent({ kanji, onKanjiClick, onKanjiClickNewTab }: Props) {
  const { translationsByChar, kanjiList } = useKanjiData();
  const { t } = useLocalization();
  const translations = translationsByChar[kanji.kanji]?.translate_examples;
  const kanjiByChar = useMemo(() => new Map(kanjiList.map(k => [k.kanji, k])), [kanjiList]);

  // Which example indices are currently showing their translation —
  // keyed per-kanji so switching to a different kanji in the same tab
  // (via a clicked example link) starts collapsed again.
  const [expanded, setExpanded] = useState<Record<number, boolean>>({});
  useEffect(() => setExpanded({}), [kanji.id]);

  const toggleExample = (i: number) =>
    setExpanded(prev => ({ ...prev, [i]: !prev[i] }));

  return (
    <>
      {/* Stroke animation */}
      <AnimatedKanji kanjiId={kanji.id} />

      {/* Header */}
      <div className="detail-header">
        <span className="detail-kanji">{kanji.kanji}</span>
        <div className="detail-header-right">
          <div className="detail-meta-badges">
            {kanji.jlpt && (
              <span className={`detail-badge jlpt-${kanji.jlpt}`}>{kanji.jlpt}</span>
            )}
            {kanji.strokes > 0 && (
              <span className="detail-badge">{kanji.strokes} {t("current_kanji.strokes")}</span>
            )}
            {kanji.grade && kanji.grade !== "0" && (
              <span className="detail-badge">{t("current_kanji.grade")} {kanji.grade}</span>
            )}
            {kanji.frequency && kanji.frequency !== "0" && (
              <span className="detail-badge">#{kanji.frequency}</span>
            )}
          </div>
        </div>
      </div>

      <ComponentsRow
        kanji={kanji}
        kanjiByChar={kanjiByChar}
        onKanjiClick={onKanjiClick}
        onKanjiClickNewTab={onKanjiClickNewTab}
      />

      {kanji.onyomi && (
        <div className="detail-section">
          <div className="detail-label">{t("current_kanji.onyomi")}</div>
          <div className="detail-reading">{kanji.onyomi}</div>
          {kanji.onyomi_romaji && (
            <div className="detail-reading-romaji">{kanji.onyomi_romaji}</div>
          )}
        </div>
      )}

      {kanji.kunyomi && (
        <div className="detail-section">
          <div className="detail-label">{t("current_kanji.kunyomi")}</div>
          <div className="detail-reading">{kanji.kunyomi}</div>
          {kanji.kunyomi_romaji && (
            <div className="detail-reading-romaji">{kanji.kunyomi_romaji}</div>
          )}
        </div>
      )}

      {kanji.meaning && (
        <div className="detail-section">
          <div className="detail-label">{t("current_kanji.meaning")}</div>
          <div className="detail-meaning">{kanji.meaning}</div>
        </div>
      )}

      {kanji.examples.length > 0 && (
        <div className="detail-section">
          <div className="detail-label">{t("current_kanji.examples")}</div>
          <div className="detail-examples">
            {kanji.examples.map((ex, i) => {
              const translation = translations?.[i];
              const isOpen = !!expanded[i] && !!translation;
              return (
                <div
                  key={i}
                  className={`detail-example${translation ? " has-translation" : ""}${isOpen ? " expanded" : ""}`}
                  onClick={() => translation && toggleExample(i)}
                >
                  <div className="detail-example-original">
                    <ExampleText text={ex} onKanjiClick={onKanjiClick} onKanjiClickNewTab={onKanjiClickNewTab} />
                  </div>
                  {translation && (
                    <div className="detail-example-translation-wrap">
                      <div className="detail-example-translation">{translation}</div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}
    </>
  );
}
