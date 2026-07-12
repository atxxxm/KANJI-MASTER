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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattens_nested_tables_into_dot_paths() {
        let toml_str = r#"
            [local.settings]
            title = "Settings"

            [local.settings.nested]
            deep = "value"
        "#;
        let value: toml::Value = toml::from_str(toml_str).unwrap();
        let mut out = HashMap::new();
        flatten(value.get("local").unwrap(), "", &mut out);

        assert_eq!(out.get("settings.title"), Some(&"Settings".to_string()));
        assert_eq!(out.get("settings.nested.deep"), Some(&"value".to_string()));
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn ignores_non_string_values_like_the_top_level_lang_code() {
        // Every locale file has a bare `lang = "en"` at the document root
        // (outside `[local]`) plus nested tables inside `[local]` — flatten()
        // is only ever called on the `local` subtree, but should still be
        // robust to non-table/non-string values (numbers, bools, arrays).
        let toml_str = r#"
            [local]
            count = 5
            enabled = true
            list = ["a", "b"]
            text = "kept"
        "#;
        let value: toml::Value = toml::from_str(toml_str).unwrap();
        let mut out = HashMap::new();
        flatten(value.get("local").unwrap(), "", &mut out);

        assert_eq!(out.len(), 1);
        assert_eq!(out.get("text"), Some(&"kept".to_string()));
    }

    #[test]
    fn real_localization_file_parses_and_flattens() {
        // Guards the actual bundled resource format end-to-end (parse + flatten),
        // without needing a Tauri AppHandle to exercise load_language() directly.
        let content = fs::read_to_string("../data/localization/English.toml")
            .expect("English.toml should exist relative to src-tauri/");
        let value: toml::Value = toml::from_str(&content).unwrap();
        let mut out = HashMap::new();
        flatten(value.get("local").unwrap(), "", &mut out);

        assert_eq!(out.get("settings.title"), Some(&"Settings".to_string()));
        assert!(out.len() > 100, "expected 100+ flattened keys, got {}", out.len());
    }
}
