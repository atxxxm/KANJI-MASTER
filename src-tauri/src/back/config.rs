use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

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
}

fn default_dark_mode() -> bool {
    true
}

fn default_interface_language() -> String {
    "English".to_string()
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