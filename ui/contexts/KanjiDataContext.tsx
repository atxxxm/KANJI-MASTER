import { createContext, useContext } from "react";
import type { KanjiDto, KanjiTranslation } from "../api/types";

export interface KanjiDataContextValue {
  kanjiList: KanjiDto[];
  loading: boolean;
  /** kanji char → its full translation entry (meaning + translate_examples). */
  translationsByChar: Record<string, KanjiTranslation>;
  translationsLoading: boolean;
  /** Re-fetches from the backend — call after anything that changes
   * meanings or example translations (Translate Kanji save, Settings
   * "Reload" button) so every tab's cached copy picks up the change. */
  refresh: () => Promise<void>;
}

export const KanjiDataContext = createContext<KanjiDataContextValue | null>(null);

export function useKanjiData(): KanjiDataContextValue {
  const ctx = useContext(KanjiDataContext);
  if (!ctx) throw new Error("useKanjiData must be used within KanjiDataContext.Provider");
  return ctx;
}
