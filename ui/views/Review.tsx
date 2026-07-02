import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SrsSummary, ReviewCard } from "../api/types";
import { useLocalization } from "../contexts/LocalizationContext";
import "../styles/review.css";

const JLPT_LEVELS = ["N5", "N4", "N3", "N2", "N1"];
const RATINGS = [
  { value: 0, key: "review.rate_again", className: "again" },
  { value: 1, key: "review.rate_hard", className: "hard" },
  { value: 2, key: "review.rate_good", className: "good" },
  { value: 3, key: "review.rate_easy", className: "easy" },
] as const;

type Phase = "setup" | "question" | "answer" | "done";

// Compact interval label for a rating button: 0 means the card repeats
// later in the same session (< 1 day). Units come from localization so
// "d/w/mo/y" translate.
function formatInterval(days: number, t: (k: string) => string): string {
  if (days < 1) return t("review.interval_soon");
  if (days < 7) return `${Math.round(days)}${t("review.unit_day")}`;
  if (days < 30) return `${Math.round(days / 7)}${t("review.unit_week")}`;
  if (days < 365) return `${Math.round(days / 30)}${t("review.unit_month")}`;
  return `${(days / 365).toFixed(1)}${t("review.unit_year")}`;
}

interface Props {
  active?: boolean;
}

export default function Review({ active }: Props) {
  const { t } = useLocalization();

  const [summary, setSummary] = useState<SrsSummary | null>(null);
  const [phase, setPhase] = useState<Phase>("setup");
  const [queue, setQueue] = useState<ReviewCard[]>([]);
  const [reviewedCount, setReviewedCount] = useState(0);
  const [previews, setPreviews] = useState<number[] | null>(null);

  const refreshSummary = useCallback(async () => {
    const s = await invoke<SrsSummary>("srs_get_summary");
    setSummary(s);
  }, []);

  useEffect(() => {
    refreshSummary();
  }, [refreshSummary]);

  const toggleLevel = async (level: string) => {
    if (!summary) return;
    const levels = summary.settings.levels.includes(level)
      ? summary.settings.levels.filter(l => l !== level)
      : [...summary.settings.levels, level];
    await invoke("srs_save_settings", { levels, newPerDay: summary.settings.new_per_day });
    await refreshSummary();
  };

  const setNewPerDay = async (newPerDay: number) => {
    if (!summary) return;
    await invoke("srs_save_settings", { levels: summary.settings.levels, newPerDay });
    await refreshSummary();
  };

  const startSession = async () => {
    const q = await invoke<ReviewCard[]>("srs_get_queue");
    if (q.length === 0) return;
    setQueue(q);
    setReviewedCount(0);
    setPhase("question");
  };

  const currentCard = queue[0] ?? null;
  const answering = useRef(false);

  // Reveals the answer and fetches the four predicted intervals for the
  // current card so the rating buttons can show "when you'll see it next".
  const reveal = useCallback(() => {
    setPhase("answer");
    const card = queue[0];
    if (!card) return;
    setPreviews(null);
    invoke<number[]>("srs_preview", { kanji: card.kanji.kanji }).then(setPreviews);
  }, [queue]);

  const answer = useCallback(async (rating: number) => {
    if (!currentCard || answering.current) return;
    answering.current = true;
    try {
      const requeue = await invoke<boolean>("srs_answer", {
        kanji: currentCard.kanji.kanji,
        rating,
      });
      setReviewedCount(c => c + 1);
      const rest = queue.slice(1);
      // Failed/learning cards come back later in the same session.
      const next = requeue ? [...rest, currentCard] : rest;
      setQueue(next);
      if (next.length === 0) {
        setPhase("done");
        refreshSummary();
      } else {
        setPhase("question");
      }
    } finally {
      answering.current = false;
    }
  }, [currentCard, queue, refreshSummary]);

  // Keyboard shortcuts while this tab is visible: Space reveals, 1-4 rate.
  useEffect(() => {
    if (!active) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if (phase === "question" && (e.key === " " || e.key === "Enter")) {
        e.preventDefault();
        reveal();
      } else if (phase === "answer" && e.key >= "1" && e.key <= "4") {
        e.preventDefault();
        answer(Number(e.key) - 1);
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [active, phase, answer, reveal]);

  if (!summary) {
    return (
      <div className="view-placeholder">
        <span>{t("common.loading")}</span>
      </div>
    );
  }

  // ── Setup / start screen ──
  if (phase === "setup" || phase === "done") {
    const nothingToStudy =
      summary.due_count === 0 &&
      (summary.new_remaining_today === 0 || summary.new_available === 0);
    return (
      <div className="review-view">
        <div className="review-setup">
          {phase === "done" && (
            <div className="review-done-banner">
              <span className="review-done-icon">🎉</span>
              <span>{t("review.session_done")}</span>
              <span className="review-done-sub">
                {reviewedCount} {t("review.answers_count")}
              </span>
            </div>
          )}

          <div className="review-stats">
            <div className="review-stat">
              <span className="review-stat-value accent">{summary.due_count}</span>
              <span className="review-stat-label">{t("review.due_today")}</span>
            </div>
            <div className="review-stat">
              <span className="review-stat-value blue">
                {Math.min(summary.new_remaining_today, summary.new_available)}
              </span>
              <span className="review-stat-label">{t("review.new_today")}</span>
            </div>
            <div className="review-stat">
              <span className="review-stat-value">{summary.total_cards}</span>
              <span className="review-stat-label">{t("review.total_cards")}</span>
            </div>
            <div className="review-stat">
              <span className="review-stat-value green">{summary.mature_count}</span>
              <span className="review-stat-label">{t("review.mature_cards")}</span>
            </div>
          </div>

          <div className="review-settings">
            <div className="review-settings-row">
              <span className="review-settings-label">{t("review.levels_label")}</span>
              <div className="review-level-btns">
                {JLPT_LEVELS.map(level => (
                  <button
                    key={level}
                    className={`filter-btn${summary.settings.levels.includes(level) ? " active" : ""}`}
                    onClick={() => toggleLevel(level)}
                  >
                    {level}
                  </button>
                ))}
              </div>
            </div>
            <div className="review-settings-row">
              <span className="review-settings-label">{t("review.new_per_day_label")}</span>
              <input
                type="range"
                min={0}
                max={50}
                step={5}
                className="settings-slider"
                value={summary.settings.new_per_day}
                onChange={e => setNewPerDay(Number(e.target.value))}
              />
              <span className="review-settings-value">{summary.settings.new_per_day}</span>
            </div>
          </div>

          <button
            className="review-start-btn"
            disabled={nothingToStudy}
            onClick={startSession}
          >
            {nothingToStudy ? t("review.all_done_today") : t("review.start_button")}
          </button>
        </div>
      </div>
    );
  }

  // ── Flashcard screen ──
  if (!currentCard) return null;
  const k = currentCard.kanji;

  return (
    <div className="review-view">
      <div className="review-session">
        <div className="review-progress">
          <span>{queue.length} {t("review.cards_left")}</span>
          {currentCard.is_new && <span className="review-new-badge">{t("review.new_badge")}</span>}
        </div>

        <div className="review-card-face">
          <span className="review-kanji">{k.kanji}</span>

          {phase === "answer" && (
            <div className="review-answer">
              {k.meaning && <div className="review-meaning">{k.meaning}</div>}
              {k.onyomi && (
                <div className="review-reading">
                  <span className="review-reading-label">{t("current_kanji.onyomi")}</span> {k.onyomi}
                </div>
              )}
              {k.kunyomi && (
                <div className="review-reading">
                  <span className="review-reading-label">{t("current_kanji.kunyomi")}</span> {k.kunyomi}
                </div>
              )}
            </div>
          )}
        </div>

        {phase === "question" ? (
          <button className="review-reveal-btn" onClick={reveal}>
            {t("review.show_answer")}
          </button>
        ) : (
          <div className="review-rating-row">
            {RATINGS.map(r => (
              <button
                key={r.value}
                className={`review-rate-btn ${r.className}`}
                onClick={() => answer(r.value)}
              >
                <span className="review-rate-label">{t(r.key)}</span>
                {previews && (
                  <span className="review-rate-interval">{formatInterval(previews[r.value], t)}</span>
                )}
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
