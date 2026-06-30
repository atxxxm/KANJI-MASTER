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
import { type Tab, createTab } from "./api/tabs";

export type View = "home" | "kanji" | "kana" | "romaji" | "draw" | "anki" | "settings";

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
  const extraProps = activeTab.kind === "settings" ? { theme, onThemeChange: setTheme } : {};

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
