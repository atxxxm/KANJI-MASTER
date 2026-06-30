import { createContext, useContext } from "react";
import type { KanjiDto } from "../api/types";

export interface KanjiDataContextValue {
  kanjiList: KanjiDto[];
  loading: boolean;
  /** Re-fetches from the backend — call after anything that changes
   * meanings (Translate Kanji save, Settings "Reload" button) so every
   * tab's cached copy picks up the change. */
  refresh: () => Promise<void>;
}

export const KanjiDataContext = createContext<KanjiDataContextValue | null>(null);

export function useKanjiData(): KanjiDataContextValue {
  const ctx = useContext(KanjiDataContext);
  if (!ctx) throw new Error("useKanjiData must be used within KanjiDataContext.Provider");
  return ctx;
}
