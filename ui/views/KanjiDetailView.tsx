import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import KanjiDetailContent from "../components/KanjiDetailContent";
import { useLocalization } from "../contexts/LocalizationContext";
import "../styles/kanji-list.css";
import "../styles/kanji-detail-view.css";

interface Props {
  kanjiChar: string;
  onOpenKanji: (k: KanjiDto) => void;
  onOpenKanjiNewTab: (k: KanjiDto) => void;
  onOpenWord: (wordId: number, wordLabel: string) => void;
}

export default function KanjiDetailView({ kanjiChar, onOpenKanji, onOpenKanjiNewTab, onOpenWord }: Props) {
  const { t } = useLocalization();
  const [kanji, setKanji] = useState<KanjiDto | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    invoke<KanjiDto | null>("get_kanji_by_char", { ch: kanjiChar }).then(k => {
      setKanji(k);
      setLoading(false);
    });
  }, [kanjiChar]);

  return (
    <div className="kanji-detail-view">
      <div className="kanji-detail-view-inner">
        {loading ? null : kanji ? (
          <KanjiDetailContent
            kanji={kanji}
            onKanjiClick={onOpenKanji}
            onKanjiClickNewTab={onOpenKanjiNewTab}
            onWordClick={onOpenWord}
          />
        ) : (
          <div className="detail-not-found">{t("home.kanji_not_found")}</div>
        )}
      </div>
    </div>
  );
}
