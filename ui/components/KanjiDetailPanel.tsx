import { invoke } from "@tauri-apps/api/core";
import { motion } from "framer-motion";
import type { KanjiDto } from "../api/types";
import AnimatedKanji from "./AnimatedKanji";

interface Props {
  kanji: KanjiDto;
  onClose: () => void;
  onKanjiClick?: (k: KanjiDto) => void;
}

// CJK Unified Ideographs range
const isCjk = (ch: string) => {
  const c = ch.charCodeAt(0);
  return c >= 0x4e00 && c <= 0x9faf;
};

function ExampleText({ text, onKanjiClick }: { text: string; onKanjiClick?: (k: KanjiDto) => void }) {
  const handleClick = async (ch: string) => {
    if (!onKanjiClick) return;
    const k = await invoke<KanjiDto | null>("get_kanji_by_char", { ch });
    if (k) onKanjiClick(k);
  };

  return (
    <>
      {[...text].map((ch, i) =>
        isCjk(ch) ? (
          <span
            key={i}
            className="example-kanji-link"
            onClick={() => handleClick(ch)}
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

export default function KanjiDetailPanel({ kanji, onClose, onKanjiClick }: Props) {
  return (
    <motion.aside
      className="kanji-detail-panel"
      initial={{ opacity: 0, x: 24 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 24 }}
      transition={{ duration: 0.18, ease: "easeOut" }}
    >
      {/* Stroke animation */}
      <AnimatedKanji kanjiId={kanji.id} />

      {/* Header */}
      <div className="detail-header">
        <span className="detail-kanji">{kanji.kanji}</span>
        <div className="detail-header-right">
          <button className="detail-close" onClick={onClose}>✕</button>
          <div className="detail-meta-badges">
            {kanji.jlpt && (
              <span className={`detail-badge jlpt-${kanji.jlpt}`}>{kanji.jlpt}</span>
            )}
            {kanji.strokes > 0 && (
              <span className="detail-badge">{kanji.strokes} strokes</span>
            )}
            {kanji.grade && kanji.grade !== "0" && (
              <span className="detail-badge">Grade {kanji.grade}</span>
            )}
            {kanji.frequency && kanji.frequency !== "0" && (
              <span className="detail-badge">#{kanji.frequency}</span>
            )}
          </div>
        </div>
      </div>

      {kanji.onyomi && (
        <div className="detail-section">
          <div className="detail-label">On-yomi</div>
          <div className="detail-reading">{kanji.onyomi}</div>
          {kanji.onyomi_romaji && (
            <div className="detail-reading-romaji">{kanji.onyomi_romaji}</div>
          )}
        </div>
      )}

      {kanji.kunyomi && (
        <div className="detail-section">
          <div className="detail-label">Kun-yomi</div>
          <div className="detail-reading">{kanji.kunyomi}</div>
          {kanji.kunyomi_romaji && (
            <div className="detail-reading-romaji">{kanji.kunyomi_romaji}</div>
          )}
        </div>
      )}

      {kanji.meaning && (
        <div className="detail-section">
          <div className="detail-label">Meaning</div>
          <div className="detail-meaning">{kanji.meaning}</div>
        </div>
      )}

      {kanji.examples.length > 0 && (
        <div className="detail-section">
          <div className="detail-label">Examples</div>
          <div className="detail-examples">
            {kanji.examples.map((ex, i) => (
              <div key={i} className="detail-example">
                <ExampleText text={ex} onKanjiClick={onKanjiClick} />
              </div>
            ))}
          </div>
        </div>
      )}
    </motion.aside>
  );
}
