import { motion } from "framer-motion";
import type { View } from "../App";
import { NAV_ITEMS, SETTINGS_NAV, type NavMeta } from "../api/navMeta";
import { useContextMenu } from "../contexts/ContextMenuContext";

interface Props {
  activeKind: View;
  onOpenTab: (v: View) => void;
  onOpenNewTab: (v: View) => void;
  theme: "dark" | "light";
  onToggleTheme: () => void;
  collapsed: boolean;
  onToggleCollapsed: () => void;
}

function NavButton({ item, active, onClick, onOpenNewTab }: {
  item: NavMeta;
  active: boolean;
  onClick: () => void;
  onOpenNewTab: (v: View) => void;
}) {
  const { open } = useContextMenu();

  return (
    <button
      className={`nav-item${active ? " active" : ""}`}
      onClick={onClick}
      onContextMenu={e => {
        e.preventDefault();
        open(e.clientX, e.clientY, [
          { label: "Open", onClick },
          { label: "Open in new tab", onClick: () => onOpenNewTab(item.view) },
        ]);
      }}
      title={item.label}
    >
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

export default function Sidebar({ activeKind, onOpenTab, onOpenNewTab, theme, onToggleTheme, collapsed, onToggleCollapsed }: Props) {
  return (
    <nav className={`sidebar${collapsed ? " collapsed" : ""}`}>
      <button className="sidebar-brand" onClick={onToggleCollapsed} title={collapsed ? "Expand sidebar" : "Collapse sidebar"}>
        <span className="sidebar-logo">漢</span>
        <span className="sidebar-title">Kanji Master</span>
      </button>

      <div className="sidebar-nav">
        {NAV_ITEMS.map(item => (
          <NavButton
            key={item.view}
            item={item}
            active={activeKind === item.view}
            onClick={() => onOpenTab(item.view)}
            onOpenNewTab={onOpenNewTab}
          />
        ))}
      </div>

      <div className="sidebar-footer">
        <NavButton
          item={SETTINGS_NAV}
          active={activeKind === "settings"}
          onClick={() => onOpenTab("settings")}
          onOpenNewTab={onOpenNewTab}
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
