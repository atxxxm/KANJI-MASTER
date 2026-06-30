import type { View } from "../App";

export interface Tab {
  id: string;
  kind: View;
  /** Only set for kind === "kanji-detail" */
  kanjiId?: number;
  kanjiChar?: string;
}

let counter = 0;
export function makeTabId(): string {
  counter += 1;
  return `tab-${Date.now()}-${counter}`;
}

export function createTab(kind: View): Tab {
  return { id: makeTabId(), kind };
}

export function createKanjiTab(kanjiId: number, kanjiChar: string): Tab {
  return { id: makeTabId(), kind: "kanji-detail", kanjiId, kanjiChar };
}
