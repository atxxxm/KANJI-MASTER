import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./styles/globals.css";
import "./styles/layout.css";
import "./styles/tabbar.css";
import Sidebar from "./components/Sidebar";
import TabBar from "./components/TabBar";
import ContextMenuProvider from "./components/ContextMenu";
import Home from "./views/Home";
import KanjiList from "./views/KanjiList";
import KanaChart from "./views/KanaChart";
import RomajiKana from "./views/RomajiKana";
import DrawSearch from "./views/DrawSearch";
import AnkiExport from "./views/AnkiExport";
import Settings from "./views/Settings";
import KanjiDetailView from "./views/KanjiDetailView";
import TranslateKanji from "./views/TranslateKanji";
import Review from "./views/Review";
import Words from "./views/Words";
import { type Tab, createTab, createKanjiTab } from "./api/tabs";
import type { KanjiDto, Config, TranslationFile, KanjiTranslation } from "./api/types";
import { SettingsContext } from "./contexts/SettingsContext";
import { KanjiDataContext } from "./contexts/KanjiDataContext";
import { LocalizationContext } from "./contexts/LocalizationContext";

export type View = "home" | "kanji" | "words" | "kana" | "review" | "romaji" | "draw" | "translate" | "anki" | "settings" | "kanji-detail";

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
  words: Words,
  review: Review,
  translate: TranslateKanji,
  anki: AnkiExport,
  settings: Settings,
  "kanji-detail": KanjiDetailView,
};

// Views that should never have more than one open tab at a time —
// reopening from the sidebar focuses the existing tab instead of duplicating it.
const SINGLETON_KINDS = new Set<View>(["home", "kanji", "words", "kana", "review", "romaji", "draw", "translate", "anki", "settings"]);

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [collapsed, setCollapsed] = useState(false);
  const [config, setConfig] = useState<Config | null>(null);

  const [tabs, setTabs] = useState<Tab[]>(() => {
    const home = createTab("home");
    return [home];
  });
  const [activeTabId, setActiveTabId] = useState<string>(() => tabs[0].id);
  const [focusKanjiSearchAt, setFocusKanjiSearchAt] = useState(0);

  const [kanjiList, setKanjiList] = useState<KanjiDto[]>([]);
  const [kanjiLoading, setKanjiLoading] = useState(true);
  const [translationsByChar, setTranslationsByChar] = useState<Record<string, KanjiTranslation>>({});
  const [translationsLoading, setTranslationsLoading] = useState(true);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  useEffect(() => {
    invoke<Config>("get_settings").then(setConfig);
  }, []);

  // Single shared kanji list, fetched once and handed to every view via
  // context — previously each of Home/KanjiList/RomajiKana/AnkiExport/
  // TranslateKanji fetched its own copy, and since all tabs now stay
  // mounted simultaneously (persistence), that meant up to 5 duplicate
  // IPC round-trips and 5 in-memory copies of the same array.
  const refreshKanji = useCallback(async () => {
    const data = await invoke<KanjiDto[]>("get_kanji_list");
    setKanjiList(data);
  }, []);

  useEffect(() => {
    refreshKanji().finally(() => setKanjiLoading(false));
  }, [refreshKanji]);

  // Fades out the static HTML splash screen (index.html) once the settings
  // and kanji dictionary the app actually needs to render have arrived,
  // instead of a fixed delay.
  useEffect(() => {
    if (kanjiLoading || !config) return;
    const splash = document.getElementById("splash");
    if (!splash) return;
    splash.classList.add("splash-hidden");
    const timeout = setTimeout(() => splash.remove(), 400);
    return () => clearTimeout(timeout);
  }, [kanjiLoading, config]);

  // Example translations (for the Kanji Detail view's translation toggle),
  // fetched from the same localization file as meanings but kept separately
  // since get_kanji_list only carries the flattened `meaning` string.
  const path_to_kanji_localization = config?.path_to_kanji_localization;
  const refreshTranslations = useCallback(async () => {
    if (!path_to_kanji_localization) return;
    const tf = await invoke<TranslationFile>("get_translations", { path: path_to_kanji_localization });
    setTranslationsByChar(tf.entries);
  }, [path_to_kanji_localization]);

  useEffect(() => {
    if (!path_to_kanji_localization) return;
    refreshTranslations().finally(() => setTranslationsLoading(false));
  }, [path_to_kanji_localization, refreshTranslations]);

  const refreshAll = useCallback(async () => {
    await Promise.all([refreshKanji(), refreshTranslations()]);
  }, [refreshKanji, refreshTranslations]);

  // Interface language: list of bundled `<Name>.toml` resources, plus the
  // flat "section.key" -> text strings for whichever one is active. English
  // is always loaded as a base layer so a key missing from another language
  // file (translation not filled in yet) falls back to English text instead
  // of a raw "section.key" string.
  const [languages, setLanguages] = useState<string[]>([]);
  const [baseStrings, setBaseStrings] = useState<Record<string, string>>({});
  const [overrideStrings, setOverrideStrings] = useState<Record<string, string>>({});

  useEffect(() => {
    invoke<string[]>("list_languages").then(setLanguages);
    invoke<Record<string, string>>("get_localization", { lang: "English" }).then(setBaseStrings);
  }, []);

  const interfaceLanguage = config?.interface_language;
  useEffect(() => {
    if (!interfaceLanguage || interfaceLanguage === "English") {
      setOverrideStrings({});
      return;
    }
    invoke<Record<string, string>>("get_localization", { lang: interfaceLanguage }).then(setOverrideStrings);
  }, [interfaceLanguage]);

  const t = useCallback(
    (key: string) => overrideStrings[key] ?? baseStrings[key] ?? key,
    [baseStrings, overrideStrings]
  );

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

  // Always creates a fresh tab, bypassing the singleton dedupe above —
  // used by Ctrl+T and the "Open in new tab" context menu action.
  const forceOpenTab = (kind: View) => {
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

  // Always creates a new kanji-detail tab, even if that kanji is already
  // open elsewhere — "Open in new tab" from a right-click.
  const openKanjiTabForce = (kanji: KanjiDto) => {
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

  const closeOtherTabs = (id: string) => {
    setTabs(prev => prev.filter(t => t.id === id));
    setActiveTabId(id);
  };

  const closeAllTabs = () => {
    const home = createTab("home");
    setTabs([home]);
    setActiveTabId(home.id);
  };

  // Global shortcuts: Ctrl+T new tab, Ctrl+W close active tab, Ctrl+F
  // open/focus the Kanji tab's search field. Escape (clear search) is
  // handled per-view via the `active` prop instead, since only that view
  // knows what "clear" means for it.
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (!e.ctrlKey) return;
      const key = e.key.toLowerCase();

      if (key === "t") {
        e.preventDefault();
        forceOpenTab("home");
      } else if (key === "w") {
        e.preventDefault();
        closeTab(activeTabId);
      } else if (key === "f") {
        e.preventDefault();
        openOrFocusTab("kanji");
        setFocusKanjiSearchAt(Date.now());
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [tabs, activeTabId]);

  // Per-tab props: a function of the tab itself, not of which tab is active —
  // every open tab stays mounted (see render below), so each needs its own props.
  const buildProps = (tab: Tab): Record<string, unknown> => {
    const props: Record<string, unknown> = {
      onOpenKanji: openKanjiTab,
      onOpenKanjiNewTab: openKanjiTabForce,
      active: tab.id === activeTabId,
    };
    if (tab.kind === "settings") {
      props.theme = theme;
      props.onThemeChange = setTheme;
    }
    if (tab.kind === "kanji-detail") {
      props.kanjiChar = tab.kanjiChar;
    }
    if (tab.kind === "kanji") {
      props.focusSearchAt = focusKanjiSearchAt;
    }
    return props;
  };

  return (
    <SettingsContext.Provider value={{ config, updateConfig }}>
    <LocalizationContext.Provider value={{ languages, t }}>
    <KanjiDataContext.Provider value={{ kanjiList, loading: kanjiLoading, translationsByChar, translationsLoading, refresh: refreshAll }}>
    <ContextMenuProvider>
      <div className="app-layout">
        <Sidebar
          activeKind={tabs.find(t => t.id === activeTabId)?.kind ?? "home"}
          onOpenTab={openOrFocusTab}
          onOpenNewTab={forceOpenTab}
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
            onCloseOthers={closeOtherTabs}
            onCloseAll={closeAllTabs}
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
    </ContextMenuProvider>
    </KanjiDataContext.Provider>
    </LocalizationContext.Provider>
    </SettingsContext.Provider>
  );
}
