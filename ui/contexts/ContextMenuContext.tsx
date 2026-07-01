import { createContext, useContext } from "react";

export interface ContextMenuItem {
  label: string;
  onClick: () => void;
  disabled?: boolean;
  danger?: boolean;
}

export type ContextMenuEntry = ContextMenuItem | "separator";

export interface ContextMenuContextValue {
  open: (x: number, y: number, items: ContextMenuEntry[]) => void;
}

export const ContextMenuContext = createContext<ContextMenuContextValue | null>(null);

export function useContextMenu(): ContextMenuContextValue {
  const ctx = useContext(ContextMenuContext);
  if (!ctx) throw new Error("useContextMenu must be used within ContextMenuProvider");
  return ctx;
}
