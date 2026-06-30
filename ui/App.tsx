import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./styles/globals.css";
import "./styles/layout.css";
import "./styles/tabbar.css";
import Sidebar from "./components/Sidebar";
import TabBar from "./components/TabBar";
import Home from "./views/Home";
import KanjiList from "./views/KanjiList";
import KanaChart from "./views/KanaChart";
import RomajiKana from "./views/RomajiKana";
import DrawSearch from "./views/DrawSearch";
import AnkiExport from "./views/AnkiExport";
import Settings from "./views/Settings";
import KanjiDetailView from "./views/KanjiDetailView";
import TranslateKanji from "./views/TranslateKanji";
import { type Tab, createTab, createKanjiTab } from "./api/tabs";
import type { KanjiDto, Config } from "./api/types";
import { SettingsContext } from "./contexts/SettingsContext";

export type View = "home" | "kanji" | "kana" | "romaji" | "draw" | "translate" | "anki" | "settings" | "kanji-detail";

type Theme = "dark" | "light";

const BASE_INTERFACE_FONT_SIZE = 14;

function getInitialTheme(): Theme {
  const saved = localStorage.getItem("theme");
  if (saved === "dark" || saved === "light") return saved;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

const VIEWS: Record<View, React.ComponentType<any>> = {
  home: Home,
  kanji: KanjiList,
  kana: KanaChart,
  romaji: RomajiKana,
  draw: DrawSearch,
  translate: TranslateKanji,
  anki: AnkiExport,
  settings: Settings,
  "kanji-detail": KanjiDetailView,
};

// Views that should never have more than one open tab at a time —
// reopening from the sidebar focuses the existing tab instead of duplicating it.
const SINGLETON_KINDS = new Set<View>(["home", "kanji", "kana", "romaji", "draw", "translate", "anki", "settings"]);

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [collapsed, setCollapsed] = useState(false);
  const [config, setConfig] = useState<Config | null>(null);

  const [tabs, setTabs] = useState<Tab[]>(() => {
    const home = createTab("home");
    return [home];
  });
  const [activeTabId, setActiveTabId] = useState<string>(() => tabs[0].id);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  useEffect(() => {
    invoke<Config>("get_settings").then(setConfig);
  }, []);

  // Apply live-tunable appearance settings as CSS so every view picks them
  // up without prop drilling. `zoom` (Chromium/WebView2-only, which is all
  // Tauri targets) scales both font size and spacing together, approximating
  // the old egui app's global UI scale slider.
  useEffect(() => {
    if (!config) return;
    document.documentElement.style.setProperty("--kanji-font-size", `${config.kanji_font_size}px`);
    document.body.style.setProperty("zoom", `${config.interface_font_size / BASE_INTERFACE_FONT_SIZE}`);
  }, [config]);

  const updateConfig = (patch: Partial<Config>) =>
    setConfig(c => (c ? { ...c, ...patch } : c));

  const toggleTheme = () => setTheme(t => t === "dark" ? "light" : "dark");

  const openOrFocusTab = (kind: View) => {
    if (SINGLETON_KINDS.has(kind)) {
      const existing = tabs.find(t => t.kind === kind);
      if (existing) {
        setActiveTabId(existing.id);
        return;
      }
    }
    const tab = createTab(kind);
    setTabs(prev => [...prev, tab]);
    setActiveTabId(tab.id);
  };

  const openKanjiTab = (kanji: KanjiDto) => {
    const existing = tabs.find(t => t.kind === "kanji-detail" && t.kanjiId === kanji.id);
    if (existing) {
      setActiveTabId(existing.id);
      return;
    }
    const tab = createKanjiTab(kanji.id, kanji.kanji);
    setTabs(prev => [...prev, tab]);
    setActiveTabId(tab.id);
  };

  const closeTab = (id: string) => {
    setTabs(prev => {
      const idx = prev.findIndex(t => t.id === id);
      if (idx === -1) return prev;
      const next = prev.filter(t => t.id !== id);

      if (next.length === 0) {
        const home = createTab("home");
        setActiveTabId(home.id);
        return [home];
      }
      if (id === activeTabId) {
        const fallback = next[idx - 1] ?? next[0];
        setActiveTabId(fallback.id);
      }
      return next;
    });
  };

  // Per-tab props: a function of the tab itself, not of which tab is active —
  // every open tab stays mounted (see render below), so each needs its own props.
  const buildProps = (tab: Tab): Record<string, unknown> => {
    const props: Record<string, unknown> = { onOpenKanji: openKanjiTab };
    if (tab.kind === "settings") {
      props.theme = theme;
      props.onThemeChange = setTheme;
    }
    if (tab.kind === "kanji-detail") {
      props.kanjiChar = tab.kanjiChar;
    }
    return props;
  };

  return (
    <SettingsContext.Provider value={{ config, updateConfig }}>
      <div className="app-layout">
        <Sidebar
          activeKind={tabs.find(t => t.id === activeTabId)?.kind ?? "home"}
          onOpenTab={openOrFocusTab}
          theme={theme}
          onToggleTheme={toggleTheme}
          collapsed={collapsed}
          onToggleCollapsed={() => setCollapsed(c => !c)}
        />
        <main className="app-content">
          <TabBar
            tabs={tabs}
            activeTabId={activeTabId}
            onSwitch={setActiveTabId}
            onClose={closeTab}
            onReorder={setTabs}
          />
          {/* All open tabs stay mounted so each keeps its own state (search
              text, filters, draw canvas, scroll position, ...) when the user
              switches away and back — only the active one is visible. */}
          <div className="tab-views">
            {tabs.map(tab => {
              const Component = VIEWS[tab.kind];
              return (
                <div
                  key={tab.id}
                  className={`tab-view-slot${tab.id === activeTabId ? " active" : ""}`}
                >
                  <Component {...buildProps(tab)} />
                </div>
              );
            })}
          </div>
        </main>
      </div>
    </SettingsContext.Provider>
  );
}
