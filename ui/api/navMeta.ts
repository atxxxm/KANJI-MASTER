import type { View } from "../App";

export interface NavMeta {
  view: View;
  icon: string;
  label: string;
  /** Closable tabs can be closed by the user; at least one tab always stays open. */
  closable: boolean;
}

export const NAV_ITEMS: NavMeta[] = [
  { view: "home",   icon: "⌂",  label: "Home",          closable: true },
  { view: "kanji",  icon: "漢", label: "Kanji",          closable: true },
  { view: "kana",   icon: "あ", label: "Kana",           closable: true },
  { view: "romaji", icon: "Aa", label: "Romaji → Kana", closable: true },
  { view: "draw",   icon: "✍", label: "Draw Search",    closable: true },
  { view: "anki",   icon: "↗", label: "Anki Export",    closable: true },
];

export const SETTINGS_NAV: NavMeta = { view: "settings", icon: "⚙", label: "Settings", closable: true };

export const NAV_BY_VIEW: Record<View, NavMeta> = Object.fromEntries(
  [...NAV_ITEMS, SETTINGS_NAV].map(item => [item.view, item])
) as Record<View, NavMeta>;
