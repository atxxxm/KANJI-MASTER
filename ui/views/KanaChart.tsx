import { useState } from "react";
import { GOJUON, DAKUTEN, HANDAKUTEN, YOON, type KanaCell } from "../api/kanaData";
import ModeSwitch from "../components/ModeSwitch";
import { useLocalization } from "../contexts/LocalizationContext";
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
  const { t } = useLocalization();

  return (
    <div className="kana-chart-view">
      <div className="kana-chart-toolbar">
        <ModeSwitch
          value={script}
          onChange={v => setScript(v as Script)}
          options={[
            { value: "hiragana", label: `${t("kana.hiragana")} あ` },
            { value: "katakana", label: `${t("kana.katakana")} ア` },
          ]}
        />
      </div>

      <div className="kana-chart-scroll">
        <div className="kana-section">
          <div className="kana-section-title">{t("kana.gojuon")}</div>
          <Grid rows={GOJUON} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">{t("kana.dakuten")}</div>
          <Grid rows={DAKUTEN} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">{t("kana.handakuten")}</div>
          <Grid rows={HANDAKUTEN} script={script} />
        </div>

        <div className="kana-section">
          <div className="kana-section-title">{t("kana.yoon")}</div>
          <Grid rows={YOON} script={script} yoon />
        </div>
      </div>
    </div>
  );
}
