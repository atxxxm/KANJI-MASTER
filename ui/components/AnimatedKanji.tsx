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

function drawStroke(ctx: CanvasRenderingContext2D, pts: number[][], upTo: number, color: string) {
  if (pts.length < 2 || upTo < 1) return;
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth   = 3;
  ctx.lineCap     = "round";
  ctx.lineJoin    = "round";
  ctx.beginPath();
  ctx.moveTo(pts[0][0] * SCALE, pts[0][1] * SCALE);
  for (let p = 1; p < upTo; p++) {
    ctx.lineTo(pts[p][0] * SCALE, pts[p][1] * SCALE);
  }
  ctx.stroke();
  ctx.restore();
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

  const ctx = canvas.getContext("2d")!;
  ctx.scale(dpr, dpr);

  const root = getComputedStyle(document.documentElement);
  const textColor   = root.getPropertyValue("--text").trim()   || "#e8e6e1";
  const accentColor = root.getPropertyValue("--accent").trim() || "#e05a3a";
  const borderColor = root.getPropertyValue("--border").trim() || "rgba(255,255,255,0.07)";

  // Completed strokes are baked into this offscreen layer once, so each
  // visible frame only needs a cheap blit instead of redrawing every
  // finished stroke (which may have hundreds of flattened points) from scratch.
  const staticCanvas = document.createElement("canvas");
  staticCanvas.width  = canvas.width;
  staticCanvas.height = canvas.height;
  const staticCtx = staticCanvas.getContext("2d")!;
  staticCtx.scale(dpr, dpr);
  drawGrid(staticCtx, borderColor);

  // Pre-compute when each stroke starts/ends (ms from animation start)
  const timeline = strokes.map((_, i) => ({
    start: i * (strokeMs + gapMs),
    end:   i * (strokeMs + gapMs) + strokeMs,
  }));
  const totalMs = (timeline.at(-1)?.end ?? 0) + 50;

  let startTs: number | null = null;
  let rafId: number;
  let cancelled = false;
  let committed = 0;

  const frame = (ts: number) => {
    if (cancelled) return;
    if (startTs === null) startTs = ts;
    const elapsed = ts - startTs;

    // Bake any strokes that finished since the last frame onto the static layer
    while (committed < strokes.length && elapsed >= timeline[committed].end) {
      drawStroke(staticCtx, strokes[committed], strokes[committed].length, textColor);
      committed++;
    }

    ctx.clearRect(0, 0, LOGICAL_SIZE, LOGICAL_SIZE);
    ctx.drawImage(staticCanvas, 0, 0, LOGICAL_SIZE, LOGICAL_SIZE);

    if (committed < strokes.length) {
      const { start } = timeline[committed];
      if (elapsed >= start) {
        const progress = (elapsed - start) / strokeMs;
        const count = Math.max(2, Math.floor(strokes[committed].length * progress));
        drawStroke(ctx, strokes[committed], count, accentColor);
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
