import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import type { KanjiDto } from "../api/types";
import "../styles/kanji-list.css";
import "../styles/anki-export.css";

type Step = 1 | 2 | 3;

interface ExportOptions {
  meaning: boolean;
  onyomi: boolean;
  kunyomi: boolean;
  jlpt: boolean;
  examples: boolean;
}

const DEFAULT_OPTIONS: ExportOptions = {
  meaning: true,
  onyomi: true,
  kunyomi: true,
  jlpt: true,
  examples: true,
};

const STEP_LABELS: Record<Step, string> = {
  1: "Select Kanji",
  2: "Review",
  3: "Export Options",
};

export default function AnkiExport() {
  const [step, setStep] = useState<Step>(1);
  const [allKanji, setAllKanji] = useState<KanjiDto[]>([]);
  const [search, setSearch] = useState("");
  const [jlpt, setJlpt] = useState<string>("all");
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());
  const [options, setOptions] = useState<ExportOptions>(DEFAULT_OPTIONS);
  const [status, setStatus] = useState<{ kind: "success" | "error"; text: string } | null>(null);

  useEffect(() => {
    invoke<KanjiDto[]>("get_kanji_list").then(setAllKanji);
  }, []);

  const filtered = useMemo(() => {
    const q = search.trim().toLowerCase();
    return allKanji.filter(k => {
      if (jlpt !== "all" && k.jlpt !== jlpt) return false;
      if (!q) return true;
      return k.kanji.includes(q) || (k.meaning?.toLowerCase().includes(q) ?? false);
    });
  }, [allKanji, search, jlpt]);

  const selected = useMemo(
    () => allKanji.filter(k => selectedIds.has(k.id)),
    [allKanji, selectedIds]
  );

  const toggleSelect = (id: number) => {
    setSelectedIds(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const removeFromSelection = (id: number) => {
    setSelectedIds(prev => {
      const next = new Set(prev);
      next.delete(id);
      return next;
    });
  };

  const clearAll = () => setSelectedIds(new Set());

  const handleExport = async () => {
    setStatus(null);
    try {
      const path = await save({
        title: "Export to Anki",
        defaultPath: "kanji-master-export.txt",
        filters: [{ name: "Tab-Separated Values", extensions: ["txt"] }],
      });
      if (!path) return;

      const lines = selected.map(k => {
        const fields: string[] = [k.kanji];
        if (options.meaning) fields.push(k.meaning ?? "");
        if (options.onyomi) fields.push(k.onyomi);
        if (options.kunyomi) fields.push(k.kunyomi);
        if (options.jlpt) fields.push(k.jlpt);
        if (options.examples) fields.push(k.examples.join("<br>"));
        return fields.join("\t");
      });

      await writeTextFile(path, lines.join("\n"));
      setStatus({ kind: "success", text: `Exported ${selected.length} kanji to ${path}` });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    }
  };

  return (
    <div className="anki-view">
      <div className="anki-steps">
        {([1, 2, 3] as Step[]).map((s, i) => (
          <div key={s} style={{ display: "flex", alignItems: "center", gap: 8 }}>
            <div className={`anki-step${step === s ? " active" : ""}${step > s ? " done" : ""}`}>
              <span className="anki-step-num">{step > s ? "✓" : s}</span>
              <span>{STEP_LABELS[s]}</span>
            </div>
            {i < 2 && <div className="anki-step-sep" />}
          </div>
        ))}
      </div>

      <div className="anki-body">
        {step === 1 && (
          <>
            <div className="anki-toolbar">
              <input
                className="search-input"
                placeholder="Search kanji or meaning…"
                value={search}
                onChange={e => setSearch(e.target.value)}
              />
              <div className="jlpt-filters">
                {["all", "N5", "N4", "N3", "N2", "N1"].map(level => (
                  <button
                    key={level}
                    className={`jlpt-btn f-${level}${jlpt === level ? " active" : ""}`}
                    onClick={() => setJlpt(level)}
                  >
                    {level === "all" ? "All" : level}
                  </button>
                ))}
              </div>
              <span className="anki-selected-count">{selectedIds.size} selected</span>
            </div>
            <div className="anki-grid-scroll">
              <div className="kanji-grid">
                {filtered.map(k => (
                  <button
                    key={k.id}
                    className={`kanji-card${selectedIds.has(k.id) ? " selected" : ""}`}
                    onClick={() => toggleSelect(k.id)}
                  >
                    <span className="kanji-char">{k.kanji}</span>
                    <div className="kanji-card-meta">
                      {k.jlpt && <span className={`jlpt-badge ${k.jlpt}`}>{k.jlpt}</span>}
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </>
        )}

        {step === 2 && (
          <div className="anki-review-scroll">
            {selected.length === 0 ? (
              <div className="view-placeholder">
                <span>No kanji selected</span>
              </div>
            ) : (
              selected.map(k => (
                <div key={k.id} className="anki-review-row">
                  <span className="anki-review-char">{k.kanji}</span>
                  <div className="anki-review-info">
                    <span className="anki-review-readings">
                      {[k.onyomi, k.kunyomi].filter(Boolean).join(" · ")}
                    </span>
                    {k.meaning && <span className="anki-review-meaning">{k.meaning}</span>}
                  </div>
                  <button className="anki-review-remove" onClick={() => removeFromSelection(k.id)}>
                    ✕
                  </button>
                </div>
              ))
            )}
          </div>
        )}

        {step === 3 && (
          <div className="anki-options-scroll">
            {(
              [
                ["meaning", "Meaning"],
                ["onyomi", "Onyomi"],
                ["kunyomi", "Kunyomi"],
                ["jlpt", "JLPT Level"],
                ["examples", "Examples (HTML)"],
              ] as [keyof ExportOptions, string][]
            ).map(([key, label]) => (
              <label key={key} className="anki-option-row">
                <input
                  type="checkbox"
                  checked={options[key]}
                  onChange={e => setOptions(o => ({ ...o, [key]: e.target.checked }))}
                />
                <span className="anki-option-label">{label}</span>
              </label>
            ))}
          </div>
        )}
      </div>

      <div className="anki-footer">
        <div>
          {step > 1 && (
            <button className="anki-footer-btn" onClick={() => setStep(s => (s - 1) as Step)}>
              Back
            </button>
          )}
        </div>

        {status && (
          <span className={`anki-status-message ${status.kind}`}>{status.text}</span>
        )}

        <div style={{ display: "flex", gap: 8 }}>
          {step === 2 && selected.length > 0 && (
            <button className="anki-footer-btn" onClick={clearAll}>Clear all</button>
          )}
          {step < 3 ? (
            <button
              className="anki-footer-btn primary"
              disabled={step === 1 && selectedIds.size === 0}
              onClick={() => setStep(s => (s + 1) as Step)}
            >
              Next
            </button>
          ) : (
            <button
              className="anki-footer-btn success"
              disabled={selected.length === 0}
              onClick={handleExport}
            >
              Export to TSV
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
