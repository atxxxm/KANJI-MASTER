import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence } from "framer-motion";
import type { KanjiDto } from "../api/types";
import KanjiDetailPanel from "../components/KanjiDetailPanel";
import "../styles/kanji-list.css";
import "../styles/romaji-kana.css";

// ── Kana utilities ────────────────────────────────────────────────────────────

const toHiragana = (s: string) =>
  [...s].map(c => {
    const code = c.charCodeAt(0);
    return code >= 0x30a1 && code <= 0x30f6 ? String.fromCharCode(code - 0x60) : c;
  }).join("");

const toKatakana = (s: string) =>
  [...s].map(c => {
    const code = c.charCodeAt(0);
    return code >= 0x3041 && code <= 0x3096 ? String.fromCharCode(code + 0x60) : c;
  }).join("");

// Extract last contiguous kana run from a string
function lastKanaWord(s: string): string {
  const matches = s.match(/[぀-ヿ]+/g);
  return matches?.at(-1) ?? "";
}

// ── Matching ──────────────────────────────────────────────────────────────────

interface Match {
  kanji: KanjiDto;
  reading: string;
  exact: boolean;
}

function findMatches(allKanji: KanjiDto[], kana: string): Match[] {
  if (!kana) return [];

  const searchH = toHiragana(kana);
  const results: Match[] = [];

  for (const k of allKanji) {
    const onyomi  = k.onyomi.split("、").map(r => r.trim()).filter(Boolean);
    const kunyomi = k.kunyomi.split("、").map(r => r.trim()).filter(Boolean)
                             .map(r => r.split(".")[0]); // strip okurigana

    let matched: { reading: string; exact: boolean } | null = null;

    for (const r of [...onyomi, ...kunyomi]) {
      const rH = toHiragana(r);
      if (rH === searchH) { matched = { reading: r, exact: true }; break; }
      if (!matched && rH.includes(searchH)) matched = { reading: r, exact: false };
    }

    if (matched) results.push({ kanji: k, ...matched });
  }

  // Exact matches first
  results.sort((a, b) => (b.exact ? 1 : 0) - (a.exact ? 1 : 0));
  return results.slice(0, 48);
}

// ── Component ─────────────────────────────────────────────────────────────────

export default function RomajiKana() {
  const [input, setInput]         = useState("");
  const [isKatakana, setIsKatakana] = useState(false);
  const [output, setOutput]       = useState("");
  const [allKanji, setAllKanji]   = useState<KanjiDto[]>([]);
  const [copied, setCopied]       = useState(false);
  const [selected, setSelected]   = useState<KanjiDto | null>(null);

  useEffect(() => {
    invoke<KanjiDto[]>("get_kanji_list").then(setAllKanji);
  }, []);

  // Live conversion
  useEffect(() => {
    if (!input) { setOutput(""); return; }
    invoke<string>("convert_romaji", { input, isKatakana, liveInput: true })
      .then(setOutput);
  }, [input, isKatakana]);

  const kanaQuery = useMemo(() => lastKanaWord(output), [output]);
  const matches   = useMemo(() => findMatches(allKanji, kanaQuery), [allKanji, kanaQuery]);

  const exactMatches   = matches.filter(m => m.exact);
  const partialMatches = matches.filter(m => !m.exact);

  const handleCopy = () => {
    if (!output) return;
    navigator.clipboard.writeText(output).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    });
  };

  return (
    <div className="romaji-layout">
      {/* ── Left: converter ── */}
      <div className="romaji-main">
        <div className="mode-toggle">
          <button
            className={`mode-btn${!isKatakana ? " active" : ""}`}
            onClick={() => setIsKatakana(false)}
          >
            Hiragana &nbsp;あ
          </button>
          <button
            className={`mode-btn${isKatakana ? " active" : ""}`}
            onClick={() => setIsKatakana(true)}
          >
            Katakana &nbsp;ア
          </button>
        </div>

        <textarea
          className="romaji-input"
          placeholder="Type romaji here… (e.g. kanji, nihongo)"
          value={input}
          onChange={e => setInput(e.target.value)}
          autoFocus
          spellCheck={false}
        />

        <div className="convert-arrow">↓</div>

        <div className="romaji-output-row">
          <div className={`romaji-output${!output ? " empty" : ""}`}>
            {output || "Kana will appear here"}
          </div>
          <button
            className={`copy-btn${copied ? " copied" : ""}`}
            onClick={handleCopy}
            disabled={!output}
          >
            {copied ? "✓ Copied" : "Copy"}
          </button>
        </div>
      </div>

      {/* ── Right: kanji assistant ── */}
      <aside className="romaji-assistant">
        <div className="assistant-header">
          <div className="assistant-title">Kanji assistant</div>
          {kanaQuery ? (
            <>
              <div className="assistant-query">
                Suggestions for: <span>{kanaQuery}</span>
              </div>
              <div className="assistant-count">
                {matches.length} match{matches.length !== 1 ? "es" : ""}
              </div>
            </>
          ) : (
            <div className="assistant-query">Type romaji to see matches</div>
          )}
        </div>

        {matches.length === 0 ? (
          <div className="assistant-empty">
            {kanaQuery ? `No kanji found for "${kanaQuery}"` : "Start typing to see suggestions"}
          </div>
        ) : (
          <div className="assistant-list">
            {exactMatches.length > 0 && (
              <>
                <div className="assistant-section-label">Exact</div>
                {exactMatches.map(m => (
                  <AssistantCard
                    key={m.kanji.id}
                    match={m}
                    selected={selected?.id === m.kanji.id}
                    onClick={() => setSelected(s => s?.id === m.kanji.id ? null : m.kanji)}
                  />
                ))}
              </>
            )}
            {partialMatches.length > 0 && (
              <>
                <div className="assistant-section-label">Partial</div>
                {partialMatches.map(m => (
                  <AssistantCard
                    key={m.kanji.id}
                    match={m}
                    selected={selected?.id === m.kanji.id}
                    onClick={() => setSelected(s => s?.id === m.kanji.id ? null : m.kanji)}
                  />
                ))}
              </>
            )}
          </div>
        )}
      </aside>

      {/* ── Detail panel ── */}
      <AnimatePresence>
        {selected && (
          <KanjiDetailPanel
            kanji={selected}
            onClose={() => setSelected(null)}
            onKanjiClick={setSelected}
          />
        )}
      </AnimatePresence>
    </div>
  );
}

// ── Assistant card ────────────────────────────────────────────────────────────

function AssistantCard({ match, selected, onClick }: {
  match: Match;
  selected: boolean;
  onClick: () => void;
}) {
  const { kanji, reading, exact } = match;
  return (
    <button
      className={`assistant-card${selected ? " selected" : ""}`}
      onClick={onClick}
    >
      <span className="assistant-kanji">{kanji.kanji}</span>
      <div className="assistant-info">
        <div className={`assistant-reading${exact ? " exact" : ""}`}>{reading}</div>
        <div className="assistant-sub">
          {[kanji.jlpt, kanji.meaning].filter(Boolean).join(" · ")}
        </div>
      </div>
      {kanji.jlpt && (
        <span className={`jlpt-badge ${kanji.jlpt}`} style={{ flexShrink: 0 }}>
          {kanji.jlpt}
        </span>
      )}
    </button>
  );
}
