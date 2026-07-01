import { motion } from "framer-motion";
import type { View } from "../App";
import { NAV_ITEMS, SETTINGS_NAV, type NavMeta } from "../api/navMeta";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useLocalization } from "../contexts/LocalizationContext";

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
  const { t } = useLocalization();
  const label = t(`nav.${item.view}`);

  return (
    <button
      className={`nav-item${active ? " active" : ""}`}
      onClick={onClick}
      onContextMenu={e => {
        e.preventDefault();
        open(e.clientX, e.clientY, [
          { label: t("context_menu.open"), onClick },
          { label: t("context_menu.open_new_tab"), onClick: () => onOpenNewTab(item.view) },
        ]);
      }}
      title={label}
    >
      {active && (
        <motion.div
          layoutId="nav-active-pill"
          className="nav-active-pill"
          transition={{ type: "spring", stiffness: 500, damping: 38 }}
        />
      )}
      <span className="nav-icon">{item.icon}</span>
      <span className="nav-label">{label}</span>
    </button>
  );
}

export default function Sidebar({ activeKind, onOpenTab, onOpenNewTab, theme, onToggleTheme, collapsed, onToggleCollapsed }: Props) {
  const { t } = useLocalization();
  return (
    <nav className={`sidebar${collapsed ? " collapsed" : ""}`}>
      <button className="sidebar-brand" onClick={onToggleCollapsed} title={collapsed ? t("nav.expand_sidebar") : t("nav.collapse_sidebar")}>
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

        <button className="nav-item" onClick={onToggleTheme} title={theme === "dark" ? t("nav.light_mode") : t("nav.dark_mode")}>
          <span className="nav-icon">{theme === "dark" ? "☀" : "☾"}</span>
          <span className="nav-label">{theme === "dark" ? t("nav.light_mode") : t("nav.dark_mode")}</span>
        </button>
      </div>
    </nav>
  );
}
