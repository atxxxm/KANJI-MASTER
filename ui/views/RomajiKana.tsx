import { useState, useMemo, useRef, useLayoutEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import ModeSwitch from "../components/ModeSwitch";
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
  onOpenKanjiNewTab: (k: KanjiDto) => void;
}

export default function RomajiKana({ onOpenKanji, onOpenKanjiNewTab }: Props) {
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

  const toggleMode = (mode: string) => {
    const katakana = mode === "katakana";
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
        <div className="romaji-main-inner">
          <div className="romaji-header">
            <div className="romaji-title">Romaji → Kana</div>
            <div className="romaji-subtitle">Type romaji and watch it convert as you go</div>
          </div>

          <ModeSwitch
            className="spaced"
            value={isKatakana ? "katakana" : "hiragana"}
            onChange={toggleMode}
            options={[
              { value: "hiragana", label: "Hiragana あ" },
              { value: "katakana", label: "Katakana ア" },
            ]}
          />

          <div className="romaji-card">
            <textarea
              ref={textareaRef}
              className="romaji-input"
              placeholder="kanji, nihongo, konnichiwa…"
              value={text}
              onChange={handleChange}
              autoFocus
              spellCheck={false}
            />
            <div className="romaji-card-footer">
              <span className="romaji-char-count">
                {text.length} character{text.length !== 1 ? "s" : ""}
              </span>
              <button
                className={`copy-btn${copied ? " copied" : ""}`}
                onClick={handleCopy}
                disabled={!text}
              >
                {copied ? "✓ Copied" : "⧉ Copy"}
              </button>
            </div>
          </div>

          <div className="romaji-hint">
            Tip: type <kbd>n'</kbd> for ん before a vowel, <kbd>-</kbd> for a long vowel (ー)
          </div>
        </div>
      </div>

      {/* ── Right: kanji assistant ── */}
      <aside className="romaji-assistant">
        <div className="assistant-header">
          <div className="assistant-title">
            <span className="assistant-title-icon">漢</span>
            Kanji assistant
          </div>
          {kanaQuery ? (
            <>
              <div className="assistant-query">
                Suggestions for <span>{kanaQuery}</span>
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
            <span className="assistant-empty-icon">{kanaQuery ? "😕" : "✎"}</span>
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
                    onOpenKanji={onOpenKanji}
                    onOpenKanjiNewTab={onOpenKanjiNewTab}
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
                    onOpenKanji={onOpenKanji}
                    onOpenKanjiNewTab={onOpenKanjiNewTab}
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

function AssistantCard({ match, onOpenKanji, onOpenKanjiNewTab }: {
  match: Match;
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
}) {
  const { open } = useContextMenu();
  const { kanji, reading, exact } = match;
  return (
    <button
      className="assistant-card"
      onClick={() => onOpenKanji(kanji)}
      onContextMenu={e => {
        e.preventDefault();
        open(e.clientX, e.clientY, kanjiMenuItems(kanji, onOpenKanji, onOpenKanjiNewTab));
      }}
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
