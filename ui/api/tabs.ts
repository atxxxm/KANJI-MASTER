import type { View } from "../App";

export interface Tab {
  id: string;
  kind: View;
}

let counter = 0;
export function makeTabId(): string {
  counter += 1;
  return `tab-${Date.now()}-${counter}`;
}

export function createTab(kind: View): Tab {
  return { id: makeTabId(), kind };
}
