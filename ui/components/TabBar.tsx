import { motion, Reorder } from "framer-motion";
import type { Tab } from "../api/tabs";
import { NAV_BY_VIEW } from "../api/navMeta";
import { useContextMenu } from "../contexts/ContextMenuContext";

interface Props {
  tabs: Tab[];
  activeTabId: string;
  onSwitch: (id: string) => void;
  onClose: (id: string) => void;
  onReorder: (tabs: Tab[]) => void;
  onCloseOthers: (id: string) => void;
  onCloseAll: () => void;
}

export default function TabBar({ tabs, activeTabId, onSwitch, onClose, onReorder, onCloseOthers, onCloseAll }: Props) {
  const { open } = useContextMenu();

  return (
    <Reorder.Group as="div" axis="x" values={tabs} onReorder={onReorder} className="tab-bar">
      {tabs.map(tab => {
        const isKanjiTab = tab.kind === "kanji-detail";
        const meta = isKanjiTab ? null : NAV_BY_VIEW[tab.kind];
        const active = tab.id === activeTabId;
        return (
          <Reorder.Item key={tab.id} value={tab} as="div" className="tab-chip-item">
            <button
              className={`tab-chip${active ? " active" : ""}`}
              onClick={() => onSwitch(tab.id)}
              onMouseDown={e => {
                // Middle-click closes the tab, like a browser
                if (e.button === 1) {
                  e.preventDefault();
                  onClose(tab.id);
                }
              }}
              onContextMenu={e => {
                e.preventDefault();
                open(e.clientX, e.clientY, [
                  { label: "Close", onClick: () => onClose(tab.id), disabled: tabs.length <= 1 },
                  { label: "Close others", onClick: () => onCloseOthers(tab.id), disabled: tabs.length <= 1 },
                  "separator",
                  { label: "Close all", onClick: onCloseAll, danger: true },
                ]);
              }}
              title={isKanjiTab ? tab.kanjiChar : meta!.label}
            >
              {active && (
                <motion.div
                  layoutId="tab-active-bg"
                  className="tab-active-bg"
                  transition={{ type: "spring", stiffness: 500, damping: 38 }}
                />
              )}
              {isKanjiTab ? (
                <span className="tab-chip-kanji">{tab.kanjiChar}</span>
              ) : (
                <>
                  <span className="tab-chip-icon">{meta!.icon}</span>
                  <span className="tab-chip-label">{meta!.label}</span>
                </>
              )}
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
          </Reorder.Item>
        );
      })}
    </Reorder.Group>
  );
}
