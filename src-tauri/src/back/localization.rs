use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, Manager};

/// Lists available interface languages by reading the bundled localization
/// resource folder — each `<Name>.toml` file there becomes one entry named
/// `<Name>` (e.g. "Русский.toml" -> "Русский").
pub fn list_available_languages(app: &AppHandle) -> Result<Vec<String>> {
    let dir = app
        .path()
        .resolve("localization", tauri::path::BaseDirectory::Resource)
        .context("Could not resolve localization resource directory")?;

    let mut names: Vec<String> = fs::read_dir(&dir)
        .with_context(|| format!("Could not read {}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                path.file_stem().and_then(|s| s.to_str()).map(String::from)
            } else {
                None
            }
        })
        .collect();

    names.sort();
    Ok(names)
}

/// Loads `<lang>.toml` from the localization resources and flattens its
/// nested tables into dot-path keys (e.g. "settings.title"), stripping the
/// single top-level `[local]` wrapper table the source files use.
pub fn load_language(app: &AppHandle, lang: &str) -> Result<HashMap<String, String>> {
    let dir = app
        .path()
        .resolve("localization", tauri::path::BaseDirectory::Resource)
        .context("Could not resolve localization resource directory")?;

    let path = dir.join(format!("{lang}.toml"));
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not open {}", path.display()))?;

    let value: toml::Value = toml::from_str(&content)
        .with_context(|| format!("TOML parsing error in {}", path.display()))?;

    let mut out = HashMap::new();
    if let Some(local) = value.get("local") {
        flatten(local, "", &mut out);
    }
    Ok(out)
}

fn flatten(value: &toml::Value, prefix: &str, out: &mut HashMap<String, String>) {
    match value {
        toml::Value::Table(table) => {
            for (k, v) in table {
                let key = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                flatten(v, &key, out);
            }
        }
        toml::Value::String(s) => {
            out.insert(prefix.to_string(), s.clone());
        }
        _ => {}
    }
}
