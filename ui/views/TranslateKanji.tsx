import { useState, useEffect, useMemo, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto, TranslationFile, KanjiTranslation } from "../api/types";
import { useSettings } from "../contexts/SettingsContext";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useLocalization } from "../contexts/LocalizationContext";
import "../styles/kanji-list.css";
import "../styles/translate.css";

const EMPTY_FILE: TranslationFile = { last_id: 0, entries: {} };

function syncBuffers(kanji: KanjiDto, data: TranslationFile): { meaning: string; examples: string[] } {
  const entry = data.entries[kanji.kanji];
  const examples = kanji.examples.map((_, i) => entry?.translate_examples[i] ?? "");
  return { meaning: entry?.meaning ?? "", examples };
}

export default function TranslateKanji() {
  const { config } = useSettings();
  const { t } = useLocalization();
  const path = config?.path_to_kanji_localization;
  const { kanjiList: allKanji, loading: kanjiLoading, refresh: refreshKanji } = useKanjiData();

  const [data, setData] = useState<TranslationFile>(EMPTY_FILE);
  const [translationsLoaded, setTranslationsLoaded] = useState(false);
  const buffersInitialized = useRef(false);

  const [currentIndex, setCurrentIndex] = useState(0);
  const [meaningBuffer, setMeaningBuffer] = useState("");
  const [examplesBuffer, setExamplesBuffer] = useState<string[]>([]);

  const [jumpQuery, setJumpQuery] = useState("");
  const [jumpFocused, setJumpFocused] = useState(false);
  const [status, setStatus] = useState<{ kind: "success" | "warn"; text: string } | null>(null);

  const jumpWrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!path) return;
    invoke<TranslationFile>("get_translations", { path }).then(tf => {
      setData(tf);
      setTranslationsLoaded(true);
    });
  }, [path]);

  // Seed the editing buffers from kanji #0 once both the kanji list and the
  // translations file have arrived — but only once, so a later refresh()
  // (e.g. from Settings reload) doesn't stomp on in-progress edits.
  useEffect(() => {
    if (buffersInitialized.current || kanjiLoading || !translationsLoaded || allKanji.length === 0) return;
    const buf = syncBuffers(allKanji[0], data);
    setMeaningBuffer(buf.meaning);
    setExamplesBuffer(buf.examples);
    buffersInitialized.current = true;
  }, [allKanji, kanjiLoading, translationsLoaded, data]);

  // Close the jump dropdown on outside click
  useEffect(() => {
    const onClick = (e: MouseEvent) => {
      if (jumpWrapRef.current && !jumpWrapRef.current.contains(e.target as Node)) {
        setJumpFocused(false);
      }
    };
    window.addEventListener("mousedown", onClick);
    return () => window.removeEventListener("mousedown", onClick);
  }, []);

  const currentKanji = allKanji[currentIndex];

  const jumpMatches = useMemo(() => {
    const q = jumpQuery.trim().toLowerCase();
    if (!q) return [];
    return allKanji
      .map((k, idx) => ({ k, idx }))
      .filter(({ k }) =>
        k.kanji.includes(q) ||
        String(k.id) === q ||
        k.onyomi_romaji.toLowerCase().includes(q) ||
        k.kunyomi_romaji.toLowerCase().includes(q) ||
        k.onyomi.includes(q) ||
        k.kunyomi.includes(q)
      )
      .slice(0, 15);
  }, [allKanji, jumpQuery]);

  // Folds the in-progress edit for `kanji` into `data`, returning the new file.
  const commitEntry = (base: TranslationFile, kanji: KanjiDto): TranslationFile => {
    const entry: KanjiTranslation = { meaning: meaningBuffer, translate_examples: examplesBuffer };
    return {
      last_id: kanji.id,
      entries: { ...base.entries, [kanji.kanji]: entry },
    };
  };

  const goTo = (index: number) => {
    if (!currentKanji) return;
    const updated = commitEntry(data, currentKanji);
    setData(updated);
    setCurrentIndex(index);
    setStatus(null);

    const target = allKanji[index];
    if (target) {
      const buf = syncBuffers(target, updated);
      setMeaningBuffer(buf.meaning);
      setExamplesBuffer(buf.examples);
    }
  };

  const handleJumpSelect = (idx: number) => {
    goTo(idx);
    setJumpQuery("");
    setJumpFocused(false);
    setStatus({ kind: "success", text: t("translate_kanji_locale.found") });
  };

  const handleSave = async () => {
    if (!path || !currentKanji) return;
    const updated = commitEntry(data, currentKanji);
    setData(updated);
    try {
      await invoke("save_translations", { path, data: updated });
      setStatus({ kind: "success", text: `${t("translate_kanji_locale.saved_at_id")} ${updated.last_id}` });
      // Propagate the new meanings to every other tab's cached kanji list.
      await refreshKanji();
    } catch (e) {
      setStatus({ kind: "warn", text: String(e) });
    }
  };

  if (!path) {
    return (
      <div className="view-placeholder">
        <span>{t("translate_kanji_locale.set_path_first")}</span>
      </div>
    );
  }

  if (kanjiLoading || !translationsLoaded) {
    return (
      <div className="view-placeholder">
        <span>{t("common.loading")}</span>
      </div>
    );
  }

  const allProcessed = currentIndex >= allKanji.length;

  return (
    <div className="translate-view">
      <div className="translate-toolbar">
        <div className="translate-jump-wrap" ref={jumpWrapRef}>
          <input
            className="search-input"
            placeholder={t("translate_kanji_locale.jump_to")}
            aria-label={t("translate_kanji_locale.jump_to")}
            value={jumpQuery}
            onChange={e => setJumpQuery(e.target.value)}
            onFocus={() => setJumpFocused(true)}
          />
          {jumpFocused && jumpQuery && jumpMatches.length > 0 && (
            <div className="translate-jump-dropdown">
              {jumpMatches.map(({ k, idx }) => (
                <button key={k.id} className="translate-jump-item" onClick={() => handleJumpSelect(idx)}>
                  <span className="translate-jump-kanji">{k.kanji}</span>
                  <span className="translate-jump-info">
                    {k.onyomi_romaji} / {k.kunyomi_romaji} · ID {k.id}
                  </span>
                </button>
              ))}
            </div>
          )}
        </div>

        <div style={{ flex: 1 }} />

        <button className="translate-save-btn" onClick={handleSave} disabled={!currentKanji}>
          💾 {t("translate_kanji_locale.save_progress_button")}
        </button>

        {status && <span className={`translate-status ${status.kind}`}>{status.text}</span>}
      </div>

      <div className="translate-scroll">
        <div className="translate-inner">
          {allProcessed || !currentKanji ? (
            <div className="translate-header">
              <span style={{ fontSize: 48 }}>🎉</span>
              <span className="translate-section-title">{t("translate_kanji_locale.all_kanji_processed")}</span>
            </div>
          ) : (
            <>
              <div className="translate-header">
                <span className="translate-kanji">{currentKanji.kanji}</span>
                <div className="translate-chips">
                  <span className="translate-chip">{t("translate_kanji_locale.id_label")}: {currentKanji.id}</span>
                  <span className="translate-chip">{t("translate_kanji_locale.index_label")}: {currentIndex + 1} / {allKanji.length}</span>
                </div>
              </div>

              <div className="translate-section-title">{t("translate_kanji_locale.meaning")}</div>
              <input
                className="translate-meaning-input"
                placeholder={t("translate_kanji_locale.hint_meaning_input")}
                value={meaningBuffer}
                onChange={e => setMeaningBuffer(e.target.value)}
              />

              <div className="translate-examples-header">
                <span className="translate-section-title" style={{ margin: 0 }}>{t("translate_kanji_locale.examples")}</span>
                <span className="translate-examples-count">({currentKanji.examples.length})</span>
              </div>

              {currentKanji.examples.map((original, i) => (
                <div key={i} className="translate-example-card">
                  <span className="translate-example-original">{t("translate_kanji_locale.original")}</span>
                  <span className="translate-example-original-text">{original}</span>
                  <div className="translate-example-divider" />
                  <div className="translate-example-label">{t("translate_kanji_locale.translation")}</div>
                  <textarea
                    className="translate-example-textarea"
                    placeholder={t("translate_kanji_locale.hint_examples_input")}
                    value={examplesBuffer[i] ?? ""}
                    onChange={e => {
                      const next = [...examplesBuffer];
                      next[i] = e.target.value;
                      setExamplesBuffer(next);
                    }}
                  />
                </div>
              ))}

              <div className="translate-nav-row">
                <button
                  className="translate-nav-btn"
                  disabled={currentIndex === 0}
                  onClick={() => goTo(currentIndex - 1)}
                >
                  ← {t("translate_kanji_locale.previous_button")}
                </button>
                <button
                  className="translate-nav-btn primary"
                  onClick={() => goTo(currentIndex + 1)}
                >
                  {t("translate_kanji_locale.next_button")} →
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
