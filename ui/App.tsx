import { useEffect, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import "./styles/globals.css";
import "./styles/layout.css";
import Sidebar from "./components/Sidebar";
import KanjiList from "./views/KanjiList";
import KanaChart from "./views/KanaChart";
import RomajiKana from "./views/RomajiKana";
import DrawSearch from "./views/DrawSearch";
import AnkiExport from "./views/AnkiExport";
import Settings from "./views/Settings";

export type View = "kanji" | "kana" | "romaji" | "draw" | "anki" | "settings";

type Theme = "dark" | "light";

function getInitialTheme(): Theme {
  const saved = localStorage.getItem("theme");
  if (saved === "dark" || saved === "light") return saved;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

const VIEWS: Record<View, React.ComponentType<any>> = {
  kanji: KanjiList,
  kana: KanaChart,
  romaji: RomajiKana,
  draw: DrawSearch,
  anki: AnkiExport,
  settings: Settings,
};

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [view, setView] = useState<View>("kanji");
  const [collapsed, setCollapsed] = useState(false);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  const toggleTheme = () => setTheme(t => t === "dark" ? "light" : "dark");

  const ActiveView = VIEWS[view];
  const extraProps = view === "settings" ? { theme, onThemeChange: setTheme } : {};

  return (
    <div className="app-layout">
      <Sidebar
        activeView={view}
        onViewChange={setView}
        theme={theme}
        onToggleTheme={toggleTheme}
        collapsed={collapsed}
        onToggleCollapsed={() => setCollapsed(c => !c)}
      />
      <main className="app-content">
        <AnimatePresence mode="wait">
          <motion.div
            key={view}
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
