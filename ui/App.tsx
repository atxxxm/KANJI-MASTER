import { useEffect, useState } from "react";
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

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [view, setView] = useState<View>("kanji");

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  const toggleTheme = () => setTheme(t => t === "dark" ? "light" : "dark");

  return (
    <div className="app-layout">
      <Sidebar
        activeView={view}
        onViewChange={setView}
        theme={theme}
        onToggleTheme={toggleTheme}
      />
      <main className="app-content">
        {view === "kanji"    && <KanjiList />}
        {view === "kana"     && <KanaChart />}
        {view === "romaji"   && <RomajiKana />}
        {view === "draw"     && <DrawSearch />}
        {view === "anki"     && <AnkiExport />}
        {view === "settings" && <Settings theme={theme} onThemeChange={setTheme} />}
      </main>
    </div>
  );
}
