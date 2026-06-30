import type { View } from "../App";

interface NavItem {
  view: View;
  icon: string;
  label: string;
}

const TOP_NAV: NavItem[] = [
  { view: "kanji",   icon: "漢", label: "Kanji" },
  { view: "kana",    icon: "あ", label: "Kana" },
  { view: "romaji",  icon: "Aa", label: "Romaji → Kana" },
  { view: "draw",    icon: "✍", label: "Draw Search" },
  { view: "anki",    icon: "↗", label: "Anki Export" },
];

interface Props {
  activeView: View;
  onViewChange: (v: View) => void;
  theme: "dark" | "light";
  onToggleTheme: () => void;
}

export default function Sidebar({ activeView, onViewChange, theme, onToggleTheme }: Props) {
  return (
    <nav className="sidebar">
      <div className="sidebar-brand">
        <span className="sidebar-logo">漢</span>
        <span className="sidebar-title">Kanji Master</span>
      </div>

      <div className="sidebar-nav">
        {TOP_NAV.map(item => (
          <button
            key={item.view}
            className={`nav-item${activeView === item.view ? " active" : ""}`}
            onClick={() => onViewChange(item.view)}
          >
            <span className="nav-icon">{item.icon}</span>
            <span className="nav-label">{item.label}</span>
          </button>
        ))}
      </div>

      <div className="sidebar-footer">
        <button
          className={`nav-item${activeView === "settings" ? " active" : ""}`}
          onClick={() => onViewChange("settings")}
        >
          <span className="nav-icon">⚙</span>
          <span className="nav-label">Settings</span>
        </button>

        <div className="nav-divider" />

        <button className="nav-item" onClick={onToggleTheme}>
          <span className="nav-icon">{theme === "dark" ? "☀" : "☾"}</span>
          <span className="nav-label">{theme === "dark" ? "Light mode" : "Dark mode"}</span>
        </button>
      </div>
    </nav>
  );
}
