import { useState, useMemo, useRef, useLayoutEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
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

interface Props {
  onOpenKanji: (k: KanjiDto) => void;
}

export default function RomajiKana({ onOpenKanji }: Props) {
  const { kanjiList: allKanji } = useKanjiData();
  const [text, setText]             = useState("");
  const [isKatakana, setIsKatakana] = useState(false);
  const [copied, setCopied]         = useState(false);

  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const pendingCursor = useRef<number | null>(null);

  // Restore caret position after a programmatic value update
  useLayoutEffect(() => {
    if (pendingCursor.current !== null && textareaRef.current) {
      const pos = pendingCursor.current;
      textareaRef.current.setSelectionRange(pos, pos);
      pendingCursor.current = null;
    }
  }, [text]);

  // Convert only the segment before the caret on each keystroke, so kana
  // already committed further in the text is left untouched.
  const handleChange = async (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const raw = e.target.value;
    const cursor = e.target.selectionStart;
    const before = raw.slice(0, cursor);
    const after = raw.slice(cursor);

    const convertedBefore = await invoke<string>("convert_romaji", {
      input: before,
      isKatakana,
      liveInput: true,
    });

    pendingCursor.current = convertedBefore.length;
    setText(convertedBefore + after);
  };

  const toggleMode = (katakana: boolean) => {
    setIsKatakana(katakana);
    setText(prev => (katakana ? toKatakana(prev) : toHiragana(prev)));
  };

  const kanaQuery = useMemo(() => lastKanaWord(text), [text]);
  const matches   = useMemo(() => findMatches(allKanji, kanaQuery), [allKanji, kanaQuery]);

  const exactMatches   = matches.filter(m => m.exact);
  const partialMatches = matches.filter(m => !m.exact);

  const handleCopy = () => {
    if (!text) return;
    navigator.clipboard.writeText(text).then(() => {
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
            onClick={() => toggleMode(false)}
          >
            Hiragana &nbsp;あ
          </button>
          <button
            className={`mode-btn${isKatakana ? " active" : ""}`}
            onClick={() => toggleMode(true)}
          >
            Katakana &nbsp;ア
          </button>
        </div>

        <div className="romaji-output-row">
          <textarea
            ref={textareaRef}
            className="romaji-input romaji-single"
            placeholder="Type romaji here… (e.g. kanji, nihongo)"
            value={text}
            onChange={handleChange}
            autoFocus
            spellCheck={false}
          />
          <button
            className={`copy-btn${copied ? " copied" : ""}`}
            onClick={handleCopy}
            disabled={!text}
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
                    onClick={() => onOpenKanji(m.kanji)}
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
                    onClick={() => onOpenKanji(m.kanji)}
                  />
                ))}
              </>
            )}
          </div>
        )}
      </aside>
    </div>
  );
}

// ── Assistant card ────────────────────────────────────────────────────────────

function AssistantCard({ match, onClick }: {
  match: Match;
  onClick: () => void;
}) {
  const { kanji, reading, exact } = match;
  return (
    <button
      className="assistant-card"
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
