import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { KanjiDto } from "../api/types";
import KanjiDetailContent from "../components/KanjiDetailContent";
import "../styles/kanji-list.css";
import "../styles/kanji-detail-view.css";

interface Props {
  kanjiChar: string;
  onOpenKanji: (k: KanjiDto) => void;
}

export default function KanjiDetailView({ kanjiChar, onOpenKanji }: Props) {
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
          <KanjiDetailContent kanji={kanji} onKanjiClick={onOpenKanji} />
        ) : (
          <div className="detail-not-found">Kanji not found</div>
        )}
      </div>
    </div>
  );
}
