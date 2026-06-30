import type { KanjiDto } from "../api/types";

interface Props {
  kanji: KanjiDto;
  onClose: () => void;
}

export default function KanjiDetailPanel({ kanji, onClose }: Props) {
  return (
    <aside className="kanji-detail-panel">
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
              <div key={i} className="detail-example">{ex}</div>
            ))}
          </div>
        </div>
      )}
    </aside>
  );
}
