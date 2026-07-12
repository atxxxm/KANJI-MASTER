import { useRef } from "react";
import { motion, Reorder } from "framer-motion";
import type { Tab } from "../api/tabs";
import { NAV_BY_VIEW } from "../api/navMeta";
import { useContextMenu } from "../contexts/ContextMenuContext";
import { useLocalization } from "../contexts/LocalizationContext";

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
  const { t } = useLocalization();
  const tabRefs = useRef<Map<string, HTMLButtonElement>>(new Map());

  // Left/Right moves focus between tab chips, Home/End jump to the ends —
  // matches the native ARIA tablist keyboard pattern.
  const focusTabAt = (index: number) => {
    const id = tabs[index]?.id;
    if (id) tabRefs.current.get(id)?.focus();
  };
  const handleTabKeyDown = (e: React.KeyboardEvent, index: number) => {
    if (e.key === "ArrowRight") {
      e.preventDefault();
      focusTabAt((index + 1) % tabs.length);
    } else if (e.key === "ArrowLeft") {
      e.preventDefault();
      focusTabAt((index - 1 + tabs.length) % tabs.length);
    } else if (e.key === "Home") {
      e.preventDefault();
      focusTabAt(0);
    } else if (e.key === "End") {
      e.preventDefault();
      focusTabAt(tabs.length - 1);
    }
  };

  return (
    <Reorder.Group
      as="div"
      axis="x"
      values={tabs}
      onReorder={onReorder}
      className="tab-bar"
      role="tablist"
      aria-label={t("nav.open_tabs")}
    >
      {tabs.map((tab, index) => {
        const isKanjiTab = tab.kind === "kanji-detail";
        const isWordTab = tab.kind === "word-detail";
        const meta = isKanjiTab || isWordTab ? null : NAV_BY_VIEW[tab.kind];
        const label = meta ? t(`nav.${meta.view}`) : "";
        const active = tab.id === activeTabId;
        const tabTitle = isKanjiTab ? tab.kanjiChar : isWordTab ? tab.wordLabel : label;
        return (
          <Reorder.Item key={tab.id} value={tab} as="div" className="tab-chip-item">
            <div className={`tab-chip${active ? " active" : ""}`}>
              {active && (
                <motion.div
                  layoutId="tab-active-bg"
                  className="tab-active-bg"
                  transition={{ type: "spring", stiffness: 500, damping: 38 }}
                />
              )}
              <button
                ref={el => {
                  if (el) tabRefs.current.set(tab.id, el);
                  else tabRefs.current.delete(tab.id);
                }}
                className="tab-chip-select"
                role="tab"
                aria-selected={active}
                tabIndex={active ? 0 : -1}
                onClick={() => onSwitch(tab.id)}
                onKeyDown={e => handleTabKeyDown(e, index)}
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
                    { label: t("context_menu.close"), onClick: () => onClose(tab.id), disabled: tabs.length <= 1 },
                    { label: t("context_menu.close_others"), onClick: () => onCloseOthers(tab.id), disabled: tabs.length <= 1 },
                    "separator",
                    { label: t("context_menu.close_all"), onClick: onCloseAll, danger: true },
                  ]);
                }}
                title={tabTitle}
              >
                {isKanjiTab ? (
                  <span className="tab-chip-kanji">{tab.kanjiChar}</span>
                ) : isWordTab ? (
                  <span className="tab-chip-kanji">{tab.wordLabel}</span>
                ) : (
                  <>
                    <span className="tab-chip-icon" aria-hidden="true">{meta!.icon}</span>
                    <span className="tab-chip-label">{label}</span>
                  </>
                )}
              </button>
              {tabs.length > 1 && (
                <button
                  className="tab-chip-close"
                  aria-label={`${t("context_menu.close")}${tabTitle ? `: ${tabTitle}` : ""}`}
                  onClick={e => {
                    e.stopPropagation();
                    onClose(tab.id);
                  }}
                >
                  ✕
                </button>
              )}
            </div>
          </Reorder.Item>
        );
      })}
    </Reorder.Group>
  );
}
