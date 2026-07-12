import { useEffect, useRef, useState } from "react";
import { useLocalization } from "../contexts/LocalizationContext";
import "../styles/onboarding.css";

interface Props {
  onFinish: () => void;
}

const STEPS = [
  { icon: "漢", titleKey: "onboarding.step1_title", bodyKey: "onboarding.step1_body" },
  { icon: "字", titleKey: "onboarding.step2_title", bodyKey: "onboarding.step2_body" },
  { icon: "詞", titleKey: "onboarding.step3_title", bodyKey: "onboarding.step3_body" },
  { icon: "復", titleKey: "onboarding.step4_title", bodyKey: "onboarding.step4_body" },
  { icon: "描", titleKey: "onboarding.step5_title", bodyKey: "onboarding.step5_body" },
  { icon: "設", titleKey: "onboarding.step6_title", bodyKey: "onboarding.step6_body" },
];

/// A short, first-run walkthrough of the app's main sections. Shown once
/// (gated by `config.onboarding_seen`) and dismissible at any point.
export default function OnboardingTour({ onFinish }: Props) {
  const { t } = useLocalization();
  const [step, setStep] = useState(0);
  const cardRef = useRef<HTMLDivElement>(null);

  const isFirst = step === 0;
  const isLast = step === STEPS.length - 1;
  const next = () => (isLast ? onFinish() : setStep(s => s + 1));
  const back = () => setStep(s => Math.max(0, s - 1));

  // Focus the card on open/step-change so screen readers announce it, and
  // trap Tab within the small set of controls instead of leaking focus into
  // the (still-mounted) app underneath.
  useEffect(() => {
    cardRef.current?.focus();
  }, []);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onFinish();
      } else if (e.key === "ArrowRight") {
        next();
      } else if (e.key === "ArrowLeft" && !isFirst) {
        back();
      } else if (e.key === "Tab" && cardRef.current) {
        const focusable = cardRef.current.querySelectorAll<HTMLElement>("button");
        if (focusable.length === 0) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];
        if (e.shiftKey && document.activeElement === first) {
          e.preventDefault();
          last.focus();
        } else if (!e.shiftKey && document.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [step, isFirst, isLast]);

  const current = STEPS[step];

  return (
    <div className="onboarding-overlay">
      <div
        ref={cardRef}
        className="onboarding-card"
        role="dialog"
        aria-modal="true"
        aria-labelledby="onboarding-title"
        tabIndex={-1}
      >
        <button className="onboarding-skip" onClick={onFinish}>
          {t("onboarding.skip")}
        </button>

        <div className="onboarding-icon" aria-hidden="true">{current.icon}</div>
        <h2 className="onboarding-title" id="onboarding-title">{t(current.titleKey)}</h2>
        <p className="onboarding-body">{t(current.bodyKey)}</p>

        <div className="onboarding-dots" role="img" aria-label={`${step + 1} / ${STEPS.length}`}>
          {STEPS.map((_, i) => (
            <span key={i} className={`onboarding-dot${i === step ? " active" : ""}`} />
          ))}
        </div>

        <div className="onboarding-actions">
          {!isFirst && (
            <button className="onboarding-btn secondary" onClick={back}>
              {t("onboarding.back")}
            </button>
          )}
          <button className="onboarding-btn primary" onClick={next}>
            {isLast ? t("onboarding.finish") : t("onboarding.next")}
          </button>
        </div>
      </div>
    </div>
  );
}
