import { useEffect, useState } from "react";
import "./styles/globals.css";

type Theme = "dark" | "light";

function getInitialTheme(): Theme {
  const saved = localStorage.getItem("theme");
  if (saved === "dark" || saved === "light") return saved;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  const toggleTheme = () => setTheme(t => t === "dark" ? "light" : "dark");

  return (
    <div style={{ height: "100%", display: "flex", alignItems: "center", justifyContent: "center", flexDirection: "column", gap: 16 }}>
      <h1 style={{ fontSize: 32, fontWeight: 700, color: "var(--accent)" }}>漢字マスター</h1>
      <p style={{ color: "var(--text-muted)", fontSize: 13 }}>Kanji Master — UI в разработке</p>
      <button
        onClick={toggleTheme}
        style={{
          marginTop: 8,
          padding: "8px 20px",
          borderRadius: "var(--radius-md)",
          background: "var(--surface-2)",
          color: "var(--text)",
          border: "1px solid var(--border)",
          fontSize: 13,
          cursor: "pointer",
          transition: "background var(--transition)",
        }}
      >
        {theme === "dark" ? "☀ Light" : "☾ Dark"}
      </button>
    </div>
  );
}
