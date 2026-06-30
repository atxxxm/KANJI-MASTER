import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "../contexts/SettingsContext";
import "../styles/settings.css";

interface Props {
  theme: "dark" | "light";
  onThemeChange: (t: "dark" | "light") => void;
}

type Status = { kind: "success" | "error"; text: string } | null;

function Toggle({ on, onChange }: { on: boolean; onChange: (v: boolean) => void }) {
  return (
    <div className={`settings-toggle${on ? " on" : ""}`} onClick={() => onChange(!on)} />
  );
}

export default function Settings({ theme, onThemeChange }: Props) {
  const { config, updateConfig } = useSettings();
  const [status, setStatus] = useState<Status>(null);
  const [reloading, setReloading] = useState(false);

  if (!config) {
    return (
      <div className="settings-view">
        <span style={{ color: "var(--text-muted)", fontSize: 13 }}>Loading settings…</span>
      </div>
    );
  }

  const handleSave = async () => {
    setStatus(null);
    try {
      await invoke("save_settings", { config });
      onThemeChange(config.dark_mode ? "dark" : "light");
      setStatus({ kind: "success", text: "Settings saved" });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    }
  };

  const handleReloadMeanings = async () => {
    setReloading(true);
    setStatus(null);
    try {
      const count = await invoke<number>("reload_meanings", { path: config.path_to_kanji_localization });
      setStatus({ kind: "success", text: `Loaded ${count} meanings` });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    } finally {
      setReloading(false);
    }
  };

  return (
    <div className="settings-view">
      <div className="settings-section">
        <div className="settings-section-title">Appearance</div>

        <div className="settings-row">
          <span className="settings-row-label">Dark mode</span>
          <div className="settings-row-control">
            <Toggle on={config.dark_mode} onChange={v => {
              updateConfig({ dark_mode: v });
              onThemeChange(v ? "dark" : "light");
            }} />
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">Interface font size</span>
          <div className="settings-row-control">
            <input
              type="range" min={10} max={22} step={1}
              className="settings-slider"
              value={config.interface_font_size}
              onChange={e => updateConfig({ interface_font_size: Number(e.target.value) })}
            />
            <span className="settings-value">{config.interface_font_size}px</span>
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">Kanji font size</span>
          <div className="settings-row-control">
            <input
              type="range" min={24} max={96} step={2}
              className="settings-slider"
              value={config.kanji_font_size}
              onChange={e => updateConfig({ kanji_font_size: Number(e.target.value) })}
            />
            <span className="settings-value">{config.kanji_font_size}px</span>
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">Stroke animation speed</span>
          <div className="settings-row-control">
            <input
              type="range" min={0.25} max={2} step={0.05}
              className="settings-slider"
              value={config.animation_speed}
              onChange={e => updateConfig({ animation_speed: Number(e.target.value) })}
            />
            <span className="settings-value">{config.animation_speed.toFixed(2)}s</span>
          </div>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-title">Behavior</div>

        <div className="settings-row">
          <span className="settings-row-label">Show kanji meaning in lists</span>
          <div className="settings-row-control">
            <Toggle on={config.show_kanji_meaning} onChange={v => updateConfig({ show_kanji_meaning: v })} />
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">Focus search field on open</span>
          <div className="settings-row-control">
            <Toggle on={config.focus_on_search} onChange={v => updateConfig({ focus_on_search: v })} />
          </div>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-title">Files &amp; data</div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">Database path</span>
          <div className="settings-row-control">
            <input
              className="settings-path-input"
              value={config.path_to_db_core}
              onChange={e => updateConfig({ path_to_db_core: e.target.value })}
            />
          </div>
        </div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">Kanji localization (meanings) JSON</span>
          <div className="settings-row-control">
            <input
              className="settings-path-input"
              value={config.path_to_kanji_localization}
              onChange={e => updateConfig({ path_to_kanji_localization: e.target.value })}
            />
            <button className="settings-secondary-btn" onClick={handleReloadMeanings} disabled={reloading}>
              {reloading ? "Loading…" : "Reload"}
            </button>
          </div>
        </div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">SVG stroke data folder</span>
          <div className="settings-row-control">
            <input
              className="settings-path-input"
              value={config.path_to_svg_images}
              onChange={e => updateConfig({ path_to_svg_images: e.target.value })}
            />
          </div>
        </div>
      </div>

      <div className="settings-footer">
        <button className="settings-save-btn" onClick={handleSave}>Save settings</button>
        {status && <span className={`settings-status ${status.kind}`}>{status.text}</span>}
      </div>
    </div>
  );
}
