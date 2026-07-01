import { createContext, useContext } from "react";
import type { Config } from "../api/types";

export interface SettingsContextValue {
  config: Config | null;
  /** Applies a live, in-memory patch (e.g. while dragging a slider). Does not persist to disk. */
  updateConfig: (patch: Partial<Config>) => void;
}

export const SettingsContext = createContext<SettingsContextValue | null>(null);

export function useSettings(): SettingsContextValue {
  const ctx = useContext(SettingsContext);
  if (!ctx) throw new Error("useSettings must be used within SettingsContext.Provider");
  return ctx;
}
