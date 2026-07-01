import { useRef, useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { kanjiMenuItems } from "../utils/kanjiMenu";
import "../styles/kanji-list.css";
import "../styles/draw-search.css";

const CANVAS_SIZE = 340;

type Point = [number, number];

interface Props {
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
  active?: boolean;
}

export default function DrawSearch({ onOpenKanji, onOpenKanjiNewTab, active }: Props) {
  const { open } = useContextMenu();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const strokesRef = useRef<Point[][]>([]);
  const drawingRef = useRef<Point[] | null>(null);

  const [strokeCount, setStrokeCount] = useState(0);
  const [results, setResults] = useState<KanjiDto[]>([]);
  const [searching, setSearching] = useState(false);

  const redraw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d")!;
    ctx.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);

    const root = getComputedStyle(document.documentElement);
    const textColor = root.getPropertyValue("--text").trim() || "#e8e6e1";
    const borderColor = root.getPropertyValue("--border").trim() || "rgba(255,255,255,0.07)";

    // Guide grid (田)
    ctx.save();
    ctx.strokeStyle = borderColor;
    ctx.lineWidth = 1;
    ctx.setLineDash([4, 6]);
    ctx.beginPath();
    ctx.moveTo(CANVAS_SIZE / 2, 0);
    ctx.lineTo(CANVAS_SIZE / 2, CANVAS_SIZE);
    ctx.moveTo(0, CANVAS_SIZE / 2);
    ctx.lineTo(CANVAS_SIZE, CANVAS_SIZE / 2);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.restore();

    const allStrokes = drawingRef.current
      ? [...strokesRef.current, drawingRef.current]
      : strokesRef.current;

    for (const stroke of allStrokes) {
      if (stroke.length < 2) continue;
      ctx.save();
      ctx.strokeStyle = textColor;
      ctx.lineWidth = 5;
      ctx.lineCap = "round";
      ctx.lineJoin = "round";
      ctx.beginPath();
      ctx.moveTo(stroke[0][0], stroke[0][1]);
      for (let i = 1; i < stroke.length; i++) {
        ctx.lineTo(stroke[i][0], stroke[i][1]);
      }
      ctx.stroke();
      ctx.restore();
    }
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = CANVAS_SIZE * dpr;
    canvas.height = CANVAS_SIZE * dpr;
    canvas.style.width = `${CANVAS_SIZE}px`;
    canvas.style.height = `${CANVAS_SIZE}px`;
    canvas.getContext("2d")!.scale(dpr, dpr);
    redraw();
  }, [redraw]);

  const runSearch = useCallback(async () => {
    if (strokesRef.current.length === 0) {
      setResults([]);
      return;
    }
    setSearching(true);
    try {
      const strokes = strokesRef.current.map(s => s.map(([x, y]) => [x, y] as [number, number]));
      const matches = await invoke<KanjiDto[]>("search_by_strokes", { strokes });
      setResults(matches);
    } finally {
      setSearching(false);
    }
  }, []);

  const getPos = (e: React.PointerEvent<HTMLCanvasElement>): Point => {
    const rect = canvasRef.current!.getBoundingClientRect();
    return [e.clientX - rect.left, e.clientY - rect.top];
  };

  const handlePointerDown = (e: React.PointerEvent<HTMLCanvasElement>) => {
    (e.target as HTMLCanvasElement).setPointerCapture(e.pointerId);
    drawingRef.current = [getPos(e)];
    redraw();
  };

  const handlePointerMove = (e: React.PointerEvent<HTMLCanvasElement>) => {
    if (!drawingRef.current) return;
    drawingRef.current.push(getPos(e));
    redraw();
  };

  const handlePointerUp = () => {
    if (!drawingRef.current) return;
    if (drawingRef.current.length >= 2) {
      strokesRef.current.push(drawingRef.current);
      setStrokeCount(strokesRef.current.length);
    }
    drawingRef.current = null;
    redraw();
    runSearch();
  };

  const handleClear = () => {
    strokesRef.current = [];
    drawingRef.current = null;
    setStrokeCount(0);
    setResults([]);
    redraw();
  };

  const handleUndo = useCallback(() => {
    if (strokesRef.current.length === 0) return;
    strokesRef.current.pop();
    setStrokeCount(strokesRef.current.length);
    redraw();
    runSearch();
  }, [redraw, runSearch]);

  useEffect(() => {
    if (!active) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key.toLowerCase() === "z") {
        e.preventDefault();
        handleUndo();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [handleUndo, active]);

  return (
      <div className="draw-main">
        <div className="draw-canvas-wrapper">
          <canvas
            ref={canvasRef}
            onPointerDown={handlePointerDown}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
          />
          {strokeCount === 0 && (
            <div className="draw-hint">Draw a kanji here with your mouse or stylus</div>
          )}
        </div>

        <div className="draw-toolbar">
          <button className="draw-btn" onClick={handleUndo} disabled={strokeCount === 0}>
            ↺ Undo (Ctrl+Z)
          </button>
          <button className="draw-btn" onClick={handleClear} disabled={strokeCount === 0}>
            ✕ Clear
          </button>
          <span className="draw-stroke-count">
            {strokeCount} stroke{strokeCount !== 1 ? "s" : ""}
          </span>
        </div>

        <div className="draw-results">
          <div className="draw-results-label">
            {searching ? "Searching…" : strokeCount === 0 ? "Best matches" : `${results.length} matches`}
          </div>
          {results.length === 0 ? (
            <div className="view-placeholder" style={{ padding: "24px 0" }}>
              <span style={{ fontSize: 13 }}>
                {strokeCount === 0 ? "Draw something to search" : "No matches yet"}
              </span>
            </div>
          ) : (
            <div className="draw-results-grid">
              {results.map((k, i) => (
                <button
                  key={k.id}
                  className="draw-result-card"
                  onClick={() => onOpenKanji(k)}
                  onContextMenu={e => {
                    e.preventDefault();
                    open(e.clientX, e.clientY, kanjiMenuItems(k, onOpenKanji, onOpenKanjiNewTab));
                  }}
                >
                  <span className="draw-result-rank">#{i + 1}</span>
                  <span className="draw-result-char">{k.kanji}</span>
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
  );
}
