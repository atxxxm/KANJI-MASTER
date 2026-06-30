import { useState } from "react";
import { GOJUON, DAKUTEN, HANDAKUTEN, YOON, type KanaCell } from "../api/kanaData";
import "../styles/romaji-kana.css";
import "../styles/kana-chart.css";

type Script = "hiragana" | "katakana";

function Grid({ rows, script, yoon = false }: { rows: KanaCell[][]; script: Script; yoon?: boolean }) {
  const idx = script === "hiragana" ? 0 : 1;
  return (
    <div className={`kana-grid${yoon ? " yoon" : ""}`}>
      {rows.flat().map((cell, i) =>
        cell ? (
          <div key={i} className="kana-cell">
            <span className="kana-cell-char">{cell[idx]}</span>
            <span className="kana-cell-romaji">{cell[2]}</span>
          </div>
        ) : (
          <div key={i} className="kana-cell empty" />
        )
      )}
    </div>
  );
}

export default function KanaChart() {
  const [script, setScript] = useState<Script>("hiragana");

  return (
    <div className="kana-chart-view">
      <div className="kana-chart-toolbar">
        <div className="mode-toggle">
          <button
            className={`mode-btn${script === "hiragana" ? " active" : ""}`}
            onClick={() => setScript("hiragana")}
          >
            Hiragana &nbsp;あ
          </button>
          <button
            className={`mode-btn${script === "katakana" ? " active" : ""}`}
            onClick={() => setScript("katakana")}
          >
            Katakana &nbsp;ア
          </button>
        </div>
      </div>

      <div className="kana-chart-scroll">
        <div className="kana-section">
          <div className="kana-section-title">Gojūon</div>
          <Grid rows={GOJUON} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">Dakuten</div>
          <Grid rows={DAKUTEN} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">Handakuten</div>
          <Grid rows={HANDAKUTEN} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">Yōon</div>
          <Grid rows={YOON} script={script} yoon />
        </div>
      </div>
    </div>
  );
}
