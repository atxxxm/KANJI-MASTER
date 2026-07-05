import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { save, open } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";
import { useSettings } from "../contexts/SettingsContext";
import { useKanjiData } from "../contexts/KanjiDataContext";
import { useLocalization } from "../contexts/LocalizationContext";
import "../styles/settings.css";

const GITHUB_URL = "https://github.com/atxxxm/KANJI-MASTER";

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
  const { refresh: refreshKanji } = useKanjiData();
  const { languages, t } = useLocalization();
  const [status, setStatus] = useState<Status>(null);
  const [reloading, setReloading] = useState(false);
  const [progressBusy, setProgressBusy] = useState(false);

  if (!config) {
    return (
      <div className="settings-view">
        <span style={{ color: "var(--text-muted)", fontSize: 13 }}>{t("settings.loading")}</span>
      </div>
    );
  }

  const handleSave = async () => {
    setStatus(null);
    try {
      await invoke("save_settings", { config });
      onThemeChange(config.dark_mode ? "dark" : "light");
      setStatus({ kind: "success", text: t("settings.saved_status") });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    }
  };

  const handleReloadMeanings = async () => {
    setReloading(true);
    setStatus(null);
    try {
      const count = await invoke<number>("reload_meanings", { path: config.path_to_kanji_localization });
      await refreshKanji();
      setStatus({ kind: "success", text: `Loaded ${count} meanings` });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    } finally {
      setReloading(false);
    }
  };

  const handleExportProgress = async () => {
    setProgressBusy(true);
    setStatus(null);
    try {
      const data = await invoke<string>("srs_export");
      const path = await save({
        title: t("settings.export_progress"),
        defaultPath: "kanji-master-progress.json",
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (!path) return;
      await writeTextFile(path, data);
      setStatus({ kind: "success", text: t("settings.progress_exported") });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    } finally {
      setProgressBusy(false);
    }
  };

  const handleImportProgress = async () => {
    if (!window.confirm(t("settings.confirm_import_progress"))) return;
    setProgressBusy(true);
    setStatus(null);
    try {
      const path = await open({
        title: t("settings.import_progress"),
        multiple: false,
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (!path) return;
      const data = await readTextFile(path);
      await invoke("srs_import", { data });
      setStatus({ kind: "success", text: t("settings.progress_imported") });
    } catch (e) {
      setStatus({ kind: "error", text: String(e) });
    } finally {
      setProgressBusy(false);
    }
  };

  return (
    <div className="settings-view">
      <div className="settings-section">
        <div className="settings-section-title">{t("settings.appearance")}</div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.interface_language")}</span>
          <div className="settings-row-control">
            <select
              className="settings-select"
              value={config.interface_language}
              onChange={e => updateConfig({ interface_language: e.target.value })}
            >
              {languages.map(lang => (
                <option key={lang} value={lang}>{lang}</option>
              ))}
            </select>
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.dark_mode")}</span>
          <div className="settings-row-control">
            <Toggle on={config.dark_mode} onChange={v => {
              updateConfig({ dark_mode: v });
              onThemeChange(v ? "dark" : "light");
            }} />
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.interface_font_size")}</span>
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
          <span className="settings-row-label">{t("settings.kanji_font_size")}</span>
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
          <span className="settings-row-label">{t("settings.animation_speed")}</span>
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
        <div className="settings-section-title">{t("settings.behavior")}</div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.show_kanji_meaning")}</span>
          <div className="settings-row-control">
            <Toggle on={config.show_kanji_meaning} onChange={v => updateConfig({ show_kanji_meaning: v })} />
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.focus_on_search")}</span>
          <div className="settings-row-control">
            <Toggle on={config.focus_on_search} onChange={v => updateConfig({ focus_on_search: v })} />
          </div>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-title">{t("settings.progress")}</div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.export_progress")}</span>
          <div className="settings-row-control">
            <button className="settings-secondary-btn" onClick={handleExportProgress} disabled={progressBusy}>
              {t("settings.export_progress")}
            </button>
          </div>
        </div>

        <div className="settings-row">
          <span className="settings-row-label">{t("settings.import_progress")}</span>
          <div className="settings-row-control">
            <button className="settings-secondary-btn" onClick={handleImportProgress} disabled={progressBusy}>
              {t("settings.import_progress")}
            </button>
          </div>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-title">{t("settings.files_and_data")}</div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">{t("settings.database_path")}</span>
          <div className="settings-row-control">
            <input
              className="settings-path-input"
              value={config.path_to_db_core}
              onChange={e => updateConfig({ path_to_db_core: e.target.value })}
            />
          </div>
        </div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">{t("settings.kanji_localization")}</span>
          <div className="settings-row-control">
            <input
              className="settings-path-input"
              value={config.path_to_kanji_localization}
              onChange={e => updateConfig({ path_to_kanji_localization: e.target.value })}
            />
            <button className="settings-secondary-btn" onClick={handleReloadMeanings} disabled={reloading}>
              {reloading ? t("settings.loading") : t("settings.reload_button")}
            </button>
          </div>
        </div>

        <div className="settings-row settings-path-row">
          <span className="settings-row-label">{t("settings.svg_folder")}</span>
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
        <button className="settings-save-btn" onClick={handleSave}>{t("settings.save_button")}</button>
        {status && <span className={`settings-status ${status.kind}`}>{status.text}</span>}
      </div>

      <div className="settings-about">
        <button className="settings-about-link" onClick={() => openUrl(GITHUB_URL)}>
          {GITHUB_URL.replace("https://", "")}
        </button>
        <span className="settings-about-license">{t("settings.license")}</span>
      </div>
    </div>
  );
}
