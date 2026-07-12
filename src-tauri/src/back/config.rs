use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub interface_font_size: f32,
    pub kanji_font_size: f32,
    pub animation_speed: f32,
    pub path_to_db_core: String,
    pub path_to_kanji_localization: String,
    pub path_to_svg_images: String,
    pub show_kanji_meaning: bool,
    pub focus_on_search: bool,
    #[serde(default = "default_dark_mode")]
    pub dark_mode: bool,
    #[serde(default = "default_interface_language")]
    pub interface_language: String,
    /// Whether the first-run onboarding tour has been shown. Defaults to
    /// `true` when missing from an existing config.toml (an upgrading user
    /// has already found their way around), but `Config::default()` — used
    /// only when no config file exists yet — sets it to `false` explicitly
    /// so genuinely new installs see the tour once.
    #[serde(default = "default_onboarding_seen")]
    pub onboarding_seen: bool,
}

fn default_dark_mode() -> bool {
    true
}

fn default_interface_language() -> String {
    "English".to_string()
}

fn default_onboarding_seen() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = get_app_config_dir();

        let path_to_db = config_dir.join("db").join("core.db");
        let path_to_kanji_loc = config_dir.join("kanji-localization").join("en.json");
        let path_to_svg = config_dir.join("kanji-svg");

        Self {
            interface_font_size: 14.0,
            kanji_font_size: 48.0,
            animation_speed: 0.75,
            path_to_db_core: path_to_db.to_string_lossy().to_string(),
            path_to_kanji_localization: path_to_kanji_loc.to_string_lossy().to_string(),
            path_to_svg_images: path_to_svg.to_string_lossy().to_string(),
            show_kanji_meaning: false,
            focus_on_search: true,
            dark_mode: true,
            interface_language: default_interface_language(),
            onboarding_seen: false,
        }
    }
}

// Returns path to config file in correct system folder
pub fn get_config_path() -> Option<PathBuf> {
    let proj = ProjectDirs::from("rs", "atom", "kanjimaster")?;
    let mut path = proj.config_dir().to_path_buf();
    path.push("config.toml");

    Some(path)
}


// Returns path to config folder in correct system folder
pub fn get_app_config_dir() -> PathBuf {
    if let Some(proj) = ProjectDirs::from("rs", "atom", "kanjimaster") {
        let config_dir = proj.config_dir();

        if !config_dir.exists() {
            let _ = fs::create_dir_all(config_dir);
        }

        return config_dir.to_path_buf();
    }

    PathBuf::from(".")
}

/// Makes the app's data usable out of the box on a clean machine.
///
/// Read-only data (SQLite DB, KanjiVG SVGs) is pointed straight at the bundled
/// resources when the configured path doesn't exist. Writable data (the
/// per-kanji meanings the user can edit) is seeded into the config dir so edits
/// persist and aren't attempted inside a read-only install directory.
///
/// Returns true if `config` was modified and should be persisted.
pub fn ensure_data_available(config: &mut Config, resource_dir: &Path) -> bool {
    let mut changed = false;
    let app_dir = get_app_config_dir();
    let data = resource_dir.join("data");

    let bundled_db = data.join("db").join("core.db");
    if !Path::new(&config.path_to_db_core).exists() && bundled_db.exists() {
        config.path_to_db_core = bundled_db.to_string_lossy().into_owned();
        changed = true;
    }

    let bundled_svg = data.join("kanji-svg");
    if !Path::new(&config.path_to_svg_images).exists() && bundled_svg.is_dir() {
        config.path_to_svg_images = bundled_svg.to_string_lossy().into_owned();
        changed = true;
    }

    // Seed editable meanings into a writable location (never overwriting edits),
    // then point the config there if it isn't already valid.
    let bundled_loc = data.join("kanji-localization");
    let local_loc = app_dir.join("kanji-localization");
    if bundled_loc.is_dir() {
        seed_dir_if_missing(&bundled_loc, &local_loc);
    }
    if !Path::new(&config.path_to_kanji_localization).exists() {
        let en = local_loc.join("en.json");
        if en.exists() {
            config.path_to_kanji_localization = en.to_string_lossy().into_owned();
            changed = true;
        }
    }

    changed
}

/// Copies files from `src` into `dst`, skipping any that already exist so user
/// edits are preserved across launches and upgrades.
fn seed_dir_if_missing(src: &Path, dst: &Path) {
    let Ok(entries) = fs::read_dir(src) else { return };
    let _ = fs::create_dir_all(dst);
    for entry in entries.flatten() {
        let from = entry.path();
        if from.is_file() {
            let to = dst.join(entry.file_name());
            if !to.exists() {
                let _ = fs::copy(&from, &to);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// A fresh, empty pair of (src, dst) temp directories, scoped under the
    /// OS temp dir with a name unique to the calling test so parallel test
    /// threads never collide. Cleaned up on drop.
    struct TempDirs {
        src: PathBuf,
        dst: PathBuf,
    }

    impl TempDirs {
        fn new(tag: &str) -> Self {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let base = std::env::temp_dir().join(format!("kanji_master_test_{tag}_{nonce}"));
            let src = base.join("src");
            let dst = base.join("dst");
            fs::create_dir_all(&src).unwrap();
            Self { src, dst }
        }
    }

    impl Drop for TempDirs {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.src.parent().unwrap());
        }
    }

    #[test]
    fn seed_dir_if_missing_copies_files_into_a_new_destination() {
        let dirs = TempDirs::new("seed_new");
        fs::write(dirs.src.join("en.json"), "{}").unwrap();
        fs::write(dirs.src.join("ru.json"), "{}").unwrap();

        seed_dir_if_missing(&dirs.src, &dirs.dst);

        assert!(dirs.dst.join("en.json").exists());
        assert!(dirs.dst.join("ru.json").exists());
    }

    #[test]
    fn seed_dir_if_missing_never_overwrites_an_existing_file() {
        // This is the whole point of the function: a user's edited
        // kanji-localization files must survive app updates that re-bundle
        // the default en.json.
        let dirs = TempDirs::new("seed_preserve");
        fs::write(dirs.src.join("en.json"), r#"{"entries":"bundled default"}"#).unwrap();
        fs::create_dir_all(&dirs.dst).unwrap();
        fs::write(dirs.dst.join("en.json"), r#"{"entries":"user edited this"}"#).unwrap();

        seed_dir_if_missing(&dirs.src, &dirs.dst);

        let content = fs::read_to_string(dirs.dst.join("en.json")).unwrap();
        assert_eq!(content, r#"{"entries":"user edited this"}"#);
    }

    #[test]
    fn seed_dir_if_missing_adds_new_files_alongside_preserved_ones() {
        // A partially-seeded dst (e.g. only en.json from an older version)
        // should still pick up newly bundled files (e.g. ru.json) without
        // touching what's already there.
        let dirs = TempDirs::new("seed_partial");
        fs::write(dirs.src.join("en.json"), "bundled").unwrap();
        fs::write(dirs.src.join("ru.json"), "bundled").unwrap();
        fs::create_dir_all(&dirs.dst).unwrap();
        fs::write(dirs.dst.join("en.json"), "user edited").unwrap();

        seed_dir_if_missing(&dirs.src, &dirs.dst);

        assert_eq!(fs::read_to_string(dirs.dst.join("en.json")).unwrap(), "user edited");
        assert_eq!(fs::read_to_string(dirs.dst.join("ru.json")).unwrap(), "bundled");
    }

    #[test]
    fn seed_dir_if_missing_is_a_noop_when_source_does_not_exist() {
        let dirs = TempDirs::new("seed_missing_src");
        let missing_src = dirs.src.join("does-not-exist");

        // Must not panic even though the source directory was never created.
        seed_dir_if_missing(&missing_src, &dirs.dst);
        assert!(!dirs.dst.exists());
    }
}

// Load config (creates default if file doesn't exist)
pub fn load_config() -> Result<Config> {
    let path = get_config_path()
        .context("Could not determine path to config file")?;

    if !path.exists() {
        let default = Config::default(); 
        save_config(&path, &default)?;
        return Ok(default);
    }

    let mut file = File::open(&path)
        .with_context(|| format!("Could not open {}", path.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)
        .with_context(|| format!("TOML parsing error in {}", path.display()))?;

    Ok(config)
}

// Save config (creates folders if needed)
pub fn save_config<T: Serialize>(path: &PathBuf, data: &T) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_str = toml::to_string_pretty(data)?;
    let mut file = File::create(path)?;

    file.write_all(toml_str.as_bytes())?;

    Ok(())
}