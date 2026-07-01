import type { KanjiDto } from "../api/types";
import type { ContextMenuEntry } from "../contexts/ContextMenuContext";

export function kanjiMenuItems(
  kanji: KanjiDto,
  onOpenKanji: (k: KanjiDto) => void,
  onOpenKanjiNewTab: (k: KanjiDto) => void,
  t: (key: string) => string
): ContextMenuEntry[] {
  return [
    { label: t("context_menu.open"), onClick: () => onOpenKanji(kanji) },
    { label: t("context_menu.open_new_tab"), onClick: () => onOpenKanjiNewTab(kanji) },
  ];
}
