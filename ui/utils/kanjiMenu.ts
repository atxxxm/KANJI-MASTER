import type { KanjiDto } from "../api/types";
import type { ContextMenuEntry } from "../contexts/ContextMenuContext";

export function kanjiMenuItems(
  kanji: KanjiDto,
  onOpenKanji: (k: KanjiDto) => void,
  onOpenKanjiNewTab: (k: KanjiDto) => void
): ContextMenuEntry[] {
  return [
    { label: "Open", onClick: () => onOpenKanji(kanji) },
    { label: "Open in new tab", onClick: () => onOpenKanjiNewTab(kanji) },
  ];
}
