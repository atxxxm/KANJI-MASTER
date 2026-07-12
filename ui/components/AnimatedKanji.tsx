import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "../contexts/SettingsContext";

interface Props {
  kanjiId: number;
}

const DEFAULT_STROKE_MS = 520;
const KVG_SIZE = 109;
const CENTER = KVG_SIZE / 2;

/// Draws a kanji's strokes in order from its KanjiVG path data. Each stroke is a
/// real <path> revealed with the stroke-dashoffset technique. The reveal is a
/// plain CSS @keyframes animation (see kanji-list.css) — the most reliable,
/// fully compositor-driven animation primitive in the app's WebView2 runtime,
/// and, being declarative, it can't be cancelled by React re-renders the way a
/// Web Animations API / requestAnimationFrame loop can. The only JS here is
/// measuring each path's exact length and feeding it plus the per-stroke timing
/// to the animation as CSS custom properties, set imperatively (never through
/// the React style prop, so re-renders can't clobber them). If the animation
/// never runs (reduced-motion, or anything unexpected), the strokes' default
/// state is the fully-drawn glyph, so the kanji is always at least correct.
export default function AnimatedKanji({ kanjiId }: Props) {
  const { config } = useSettings();
  // animation_speed is the duration of a single stroke, in seconds.
  const strokeMs = config ? config.animation_speed * 1000 : DEFAULT_STROKE_MS;
  const gapMs = Math.min(120, strokeMs * 0.15);

  const [paths, setPaths] = useState<string[] | null>(null);
  const [hasData, setHasData] = useState(true);
  const groupRef = useRef<SVGGElement>(null);
  // Bumped on load and on Replay; used as the group's key so remounting the
  // paths restarts every stroke's CSS animation from the beginning.
  const [runId, setRunId] = useState(0);

  useEffect(() => {
    setPaths(null);
    setHasData(true);
    invoke<string[] | null>("get_svg_strokes", { kanjiId }).then(data => {
      if (data && data.length > 0) {
        setPaths(data);
        setRunId(id => id + 1);
      } else {
        setHasData(false);
      }
    });
  }, [kanjiId]);

  // Runs before paint so the animation's first frame already has the right
  // length and timing. Sets everything imperatively on the DOM nodes.
  useLayoutEffect(() => {
    const group = groupRef.current;
    if (!group || !paths) return;
    const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    group.querySelectorAll<SVGPathElement>("path").forEach((p, i) => {
      const len = p.getTotalLength();
      p.style.setProperty("--len", String(len));
      p.style.strokeDasharray = String(len);
      if (reduce) {
        p.style.strokeDashoffset = "0";
        return;
      }
      // Retrigger the animation from scratch (needed on Replay when the node is
      // reused): clear it, force a reflow so the browser notices, then set it.
      p.style.animation = "none";
      void p.getBoundingClientRect();
      p.style.animation = `kanji-draw ${strokeMs}ms ease-in-out ${i * (strokeMs + gapMs)}ms both`;
    });
  }, [paths, runId, strokeMs, gapMs]);

  if (!hasData) return null;

  return (
    <div className="anim-kanji-wrapper">
      <svg className="anim-kanji-svg" viewBox={`0 0 ${KVG_SIZE} ${KVG_SIZE}`} aria-hidden="true">
        <line className="anim-guide" x1={CENTER} y1={0} x2={CENTER} y2={KVG_SIZE} />
        <line className="anim-guide" x1={0} y1={CENTER} x2={KVG_SIZE} y2={CENTER} />
        {paths && (
          <g key={runId} ref={groupRef} className="anim-strokes">
            {paths.map((d, i) => (
              <path key={i} className="anim-stroke" d={d} />
            ))}
          </g>
        )}
      </svg>
      {paths && (
        <button className="replay-btn" onClick={() => setRunId(id => id + 1)}>▶ Replay</button>
      )}
    </div>
  );
}
