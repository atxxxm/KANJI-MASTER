import { useState, useCallback, useEffect, useRef, type ReactNode } from "react";
import { ContextMenuContext, type ContextMenuEntry } from "../contexts/ContextMenuContext";
import "../styles/context-menu.css";

interface MenuState {
  x: number;
  y: number;
  items: ContextMenuEntry[];
}

const MENU_WIDTH = 200;
const ITEM_HEIGHT = 32;

/**
 * Wraps the app tree, suppresses the native WebView2 right-click menu
 * (Back/Reload/Save as/Print/Inspect don't apply to a desktop app), and
 * renders our own menu wherever a component calls useContextMenu().open(...).
 */
export default function ContextMenuProvider({ children }: { children: ReactNode }) {
  const [menu, setMenu] = useState<MenuState | null>(null);
  const menuRef = useRef<HTMLDivElement>(null);

  const open = useCallback((x: number, y: number, items: ContextMenuEntry[]) => {
    setMenu({ x, y, items });
  }, []);

  const close = useCallback(() => setMenu(null), []);

  useEffect(() => {
    const onContextMenu = (e: MouseEvent) => e.preventDefault();
    window.addEventListener("contextmenu", onContextMenu);
    return () => window.removeEventListener("contextmenu", onContextMenu);
  }, []);

  const menuItemEls = () =>
    menuRef.current
      ? Array.from(menuRef.current.querySelectorAll<HTMLButtonElement>(".context-menu-item:not(:disabled)"))
      : [];

  // Move focus into the menu as soon as it opens, so arrow keys work without
  // requiring a prior Tab press.
  useEffect(() => {
    if (!menu) return;
    menuItemEls()[0]?.focus();
  }, [menu]);

  useEffect(() => {
    if (!menu) return;
    const onMouseDown = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) close();
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        close();
        return;
      }
      if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;
      const items = menuItemEls();
      if (items.length === 0) return;
      e.preventDefault();
      const current = items.indexOf(document.activeElement as HTMLButtonElement);
      const next = e.key === "ArrowDown"
        ? (current + 1) % items.length
        : (current - 1 + items.length) % items.length;
      items[next]?.focus();
    };
    window.addEventListener("mousedown", onMouseDown);
    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("blur", close);
    window.addEventListener("resize", close);
    return () => {
      window.removeEventListener("mousedown", onMouseDown);
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("blur", close);
      window.removeEventListener("resize", close);
    };
  }, [menu, close]);

  const style = menu ? clampToViewport(menu.x, menu.y, menu.items.length) : undefined;

  return (
    <ContextMenuContext.Provider value={{ open }}>
      {children}
      {menu && (
        <div ref={menuRef} className="context-menu" role="menu" style={style}>
          {menu.items.map((item, i) =>
            item === "separator" ? (
              <div key={i} className="context-menu-separator" role="separator" />
            ) : (
              <button
                key={i}
                role="menuitem"
                className={`context-menu-item${item.danger ? " danger" : ""}`}
                disabled={item.disabled}
                onClick={() => {
                  item.onClick();
                  close();
                }}
              >
                {item.label}
              </button>
            )
          )}
        </div>
      )}
    </ContextMenuContext.Provider>
  );
}

function clampToViewport(x: number, y: number, itemCount: number) {
  const estHeight = itemCount * ITEM_HEIGHT + 8;
  const left = Math.min(x, window.innerWidth - MENU_WIDTH - 8);
  const top = Math.min(y, window.innerHeight - estHeight - 8);
  return { left: Math.max(4, left), top: Math.max(4, top) };
}
