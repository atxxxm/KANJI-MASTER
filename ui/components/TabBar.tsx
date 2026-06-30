import { motion } from "framer-motion";
import type { Tab } from "../api/tabs";
import { NAV_BY_VIEW } from "../api/navMeta";

interface Props {
  tabs: Tab[];
  activeTabId: string;
  onSwitch: (id: string) => void;
  onClose: (id: string) => void;
}

export default function TabBar({ tabs, activeTabId, onSwitch, onClose }: Props) {
  return (
    <div className="tab-bar">
      {tabs.map(tab => {
        const meta = NAV_BY_VIEW[tab.kind];
        const active = tab.id === activeTabId;
        return (
          <button
            key={tab.id}
            className={`tab-chip${active ? " active" : ""}`}
            onClick={() => onSwitch(tab.id)}
            onMouseDown={e => {
              // Middle-click closes the tab, like a browser
              if (e.button === 1) {
                e.preventDefault();
                onClose(tab.id);
              }
            }}
            title={meta.label}
          >
            {active && (
              <motion.div
                layoutId="tab-active-bg"
                className="tab-active-bg"
                transition={{ type: "spring", stiffness: 500, damping: 38 }}
              />
            )}
            <span className="tab-chip-icon">{meta.icon}</span>
            <span className="tab-chip-label">{meta.label}</span>
            {tabs.length > 1 && (
              <span
                className="tab-chip-close"
                onClick={e => {
                  e.stopPropagation();
                  onClose(tab.id);
                }}
              >
                ✕
              </span>
            )}
          </button>
        );
      })}
    </div>
  );
}
