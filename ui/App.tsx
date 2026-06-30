import { useEffect, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
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
import { type Tab, createTab, createKanjiTab } from "./api/tabs";
import type { KanjiDto } from "./api/types";

export type View = "home" | "kanji" | "kana" | "romaji" | "draw" | "anki" | "settings" | "kanji-detail";

type Theme = "dark" | "light";

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
  anki: AnkiExport,
  settings: Settings,
  "kanji-detail": KanjiDetailView,
};

// Views that should never have more than one open tab at a time —
// reopening from the sidebar focuses the existing tab instead of duplicating it.
const SINGLETON_KINDS = new Set<View>(["home", "kanji", "kana", "romaji", "draw", "anki", "settings"]);

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [collapsed, setCollapsed] = useState(false);

  const [tabs, setTabs] = useState<Tab[]>(() => {
    const home = createTab("home");
    return [home];
  });
  const [activeTabId, setActiveTabId] = useState<string>(() => tabs[0].id);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

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

  const activeTab = tabs.find(t => t.id === activeTabId) ?? tabs[0];
  const ActiveView = VIEWS[activeTab.kind];

  const extraProps: Record<string, unknown> = { onOpenKanji: openKanjiTab };
  if (activeTab.kind === "settings") {
    extraProps.theme = theme;
    extraProps.onThemeChange = setTheme;
  }
  if (activeTab.kind === "kanji-detail") {
    extraProps.kanjiChar = activeTab.kanjiChar;
  }

  return (
    <div className="app-layout">
      <Sidebar
        activeKind={activeTab.kind}
        onOpenTab={openOrFocusTab}
        theme={theme}
        onToggleTheme={toggleTheme}
        collapsed={collapsed}
        onToggleCollapsed={() => setCollapsed(c => !c)}
      />
      <main className="app-content">
        <TabBar
          tabs={tabs}
          activeTabId={activeTab.id}
          onSwitch={setActiveTabId}
          onClose={closeTab}
          onReorder={setTabs}
        />
        <AnimatePresence mode="wait">
          <motion.div
            key={activeTab.id}
            className="view-transition"
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -6 }}
            transition={{ duration: 0.15, ease: "easeOut" }}
          >
            <ActiveView {...extraProps} />
          </motion.div>
        </AnimatePresence>
      </main>
    </div>
  );
}
