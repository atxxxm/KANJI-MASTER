import { createContext, useContext } from "react";

export interface LocalizationContextValue {
  /** Interface language names available (bundled `<Name>.toml` resources). */
  languages: string[];
  /** Looks up a "section.key" string in the active language, falling back
   * to the key itself if missing (so an un-migrated screen still renders
   * something readable instead of crashing). */
  t: (key: string) => string;
}

export const LocalizationContext = createContext<LocalizationContextValue | null>(null);

export function useLocalization(): LocalizationContextValue {
  const ctx = useContext(LocalizationContext);
  if (!ctx) throw new Error("useLocalization must be used within LocalizationContext.Provider");
  return ctx;
}
