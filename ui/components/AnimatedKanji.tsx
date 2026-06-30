import { useRef, useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  kanjiId: number;
  strokeMs?: number;  // duration per stroke in ms
  gapMs?: number;     // pause between strokes in ms
}

const LOGICAL_SIZE = 200;
const KVG_SIZE = 109;
const SCALE = LOGICAL_SIZE / KVG_SIZE;

function drawGrid(ctx: CanvasRenderingContext2D, borderColor: string) {
  ctx.save();
  ctx.strokeStyle = borderColor;
  ctx.lineWidth = 0.5;
  ctx.strokeRect(0.5, 0.5, LOGICAL_SIZE - 1, LOGICAL_SIZE - 1);
  ctx.beginPath();
  ctx.setLineDash([3, 5]);
  ctx.moveTo(LOGICAL_SIZE / 2, 0);
  ctx.lineTo(LOGICAL_SIZE / 2, LOGICAL_SIZE);
  ctx.moveTo(0, LOGICAL_SIZE / 2);
  ctx.lineTo(LOGICAL_SIZE, LOGICAL_SIZE / 2);
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.restore();
}

// Draws pts[fromIdx .. toIdx) as a single continuous segment, continuing
// visually from the point already on screen at fromIdx-1. Never touches
// pixels outside this new segment, so frames cost O(new points) instead
// of O(all points drawn so far).
function drawSegment(ctx: CanvasRenderingContext2D, pts: number[][], fromIdx: number, toIdx: number, color: string) {
  if (toIdx <= fromIdx) return;
  const startIdx = Math.max(0, fromIdx - 1);
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth   = 3;
  ctx.lineCap     = "round";
  ctx.lineJoin    = "round";
  ctx.beginPath();
  ctx.moveTo(pts[startIdx][0] * SCALE, pts[startIdx][1] * SCALE);
  for (let p = startIdx + 1; p < toIdx; p++) {
    ctx.lineTo(pts[p][0] * SCALE, pts[p][1] * SCALE);
  }
  ctx.stroke();
  ctx.restore();
}

function drawFullStroke(ctx: CanvasRenderingContext2D, pts: number[][], color: string) {
  drawSegment(ctx, pts, 1, pts.length, color);
}

function runAnimation(
  canvas: HTMLCanvasElement,
  strokes: number[][][],
  strokeMs: number,
  gapMs: number,
): () => void {
  const dpr = window.devicePixelRatio || 1;
  canvas.width  = LOGICAL_SIZE * dpr;
  canvas.height = LOGICAL_SIZE * dpr;
  canvas.style.width  = `${LOGICAL_SIZE}px`;
  canvas.style.height = `${LOGICAL_SIZE}px`;

  const ctx = canvas.getContext("2d", { alpha: false })!;
  ctx.scale(dpr, dpr);

  const root = getComputedStyle(document.documentElement);
  const textColor   = root.getPropertyValue("--text").trim()   || "#e8e6e1";
  const accentColor = root.getPropertyValue("--accent").trim() || "#e05a3a";
  const borderColor = root.getPropertyValue("--border").trim() || "rgba(255,255,255,0.07)";
  const bgColor     = root.getPropertyValue("--surface-2").trim() || "#2a2a32";

  ctx.fillStyle = bgColor;
  ctx.fillRect(0, 0, LOGICAL_SIZE, LOGICAL_SIZE);
  drawGrid(ctx, borderColor);

  // Pre-compute when each stroke starts/ends (ms from animation start)
  const timeline = strokes.map((_, i) => ({
    start: i * (strokeMs + gapMs),
    end:   i * (strokeMs + gapMs) + strokeMs,
  }));
  const totalMs = (timeline.at(-1)?.end ?? 0) + 50;

  let startTs: number | null = null;
  let rafId: number;
  let cancelled = false;
  let strokeIndex = 0;
  let drawnCount = 0; // points drawn so far within the active stroke

  const frame = (ts: number) => {
    if (cancelled) return;
    if (startTs === null) startTs = ts;
    const elapsed = ts - startTs;

    // Finalize any strokes that finished since the last frame
    while (strokeIndex < strokes.length && elapsed >= timeline[strokeIndex].end) {
      const pts = strokes[strokeIndex];
      // Top up any remaining segment, then repaint the whole stroke in its
      // resting color so accent/in-progress pixels don't linger.
      drawFullStroke(ctx, pts, textColor);
      strokeIndex++;
      drawnCount = 0;
    }

    if (strokeIndex < strokes.length) {
      const { start } = timeline[strokeIndex];
      if (elapsed >= start) {
        const pts = strokes[strokeIndex];
        const progress = (elapsed - start) / strokeMs;
        const targetCount = Math.max(2, Math.min(pts.length, Math.floor(pts.length * progress)));
        if (targetCount > drawnCount) {
          drawSegment(ctx, pts, drawnCount, targetCount, accentColor);
          drawnCount = targetCount;
        }
      }
    }

    if (elapsed < totalMs) {
      rafId = requestAnimationFrame(frame);
    }
  };

  rafId = requestAnimationFrame(frame);
  return () => {
    cancelled = true;
    cancelAnimationFrame(rafId);
  };
}

export default function AnimatedKanji({ kanjiId, strokeMs = 520, gapMs = 80 }: Props) {
  const canvasRef  = useRef<HTMLCanvasElement>(null);
  const cancelRef  = useRef<(() => void) | null>(null);
  const [strokes, setStrokes]   = useState<number[][][] | null>(null);
  const [hasData, setHasData]   = useState(true);

  useEffect(() => {
    setStrokes(null);
    setHasData(true);
    invoke<number[][][] | null>("get_svg_strokes", { kanjiId }).then(data => {
      if (data && data.length > 0) setStrokes(data);
      else setHasData(false);
    });
  }, [kanjiId]);

  const play = useCallback(() => {
    if (!strokes || !canvasRef.current) return;
    cancelRef.current?.();
    cancelRef.current = runAnimation(canvasRef.current, strokes, strokeMs, gapMs);
  }, [strokes, strokeMs, gapMs]);

  useEffect(() => {
    play();
    return () => cancelRef.current?.();
  }, [play]);

  if (!hasData) return null;

  return (
    <div className="anim-kanji-wrapper">
      <canvas ref={canvasRef} />
      {strokes && (
        <button className="replay-btn" onClick={play}>▶ Replay</button>
      )}
    </div>
  );
}
