import { motion } from "framer-motion";
import type { View } from "../App";

interface NavItem {
  view: View;
  icon: string;
  label: string;
}

const TOP_NAV: NavItem[] = [
  { view: "home",    icon: "⌂", label: "Home" },
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
  collapsed: boolean;
  onToggleCollapsed: () => void;
}

function NavButton({ item, active, onClick }: { item: NavItem; active: boolean; onClick: () => void }) {
  return (
    <button className={`nav-item${active ? " active" : ""}`} onClick={onClick} title={item.label}>
      {active && (
        <motion.div
          layoutId="nav-active-pill"
          className="nav-active-pill"
          transition={{ type: "spring", stiffness: 500, damping: 38 }}
        />
      )}
      <span className="nav-icon">{item.icon}</span>
      <span className="nav-label">{item.label}</span>
    </button>
  );
}

export default function Sidebar({ activeView, onViewChange, theme, onToggleTheme, collapsed, onToggleCollapsed }: Props) {
  return (
    <nav className={`sidebar${collapsed ? " collapsed" : ""}`}>
      <button className="sidebar-brand" onClick={onToggleCollapsed} title={collapsed ? "Expand sidebar" : "Collapse sidebar"}>
        <span className="sidebar-logo">漢</span>
        <span className="sidebar-title">Kanji Master</span>
      </button>

      <div className="sidebar-nav">
        {TOP_NAV.map(item => (
          <NavButton
            key={item.view}
            item={item}
            active={activeView === item.view}
            onClick={() => onViewChange(item.view)}
          />
        ))}
      </div>

      <div className="sidebar-footer">
        <NavButton
          item={{ view: "settings", icon: "⚙", label: "Settings" }}
          active={activeView === "settings"}
          onClick={() => onViewChange("settings")}
        />

        <div className="nav-divider" />

        <button className="nav-item" onClick={onToggleTheme} title={theme === "dark" ? "Light mode" : "Dark mode"}>
          <span className="nav-icon">{theme === "dark" ? "☀" : "☾"}</span>
          <span className="nav-label">{theme === "dark" ? "Light mode" : "Dark mode"}</span>
        </button>
      </div>
    </nav>
  );
}
