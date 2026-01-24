use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::{fs::File, io::{Read, Write}};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub interface_font_size: f32,
    pub kanji_font_size: f32, 
    pub auto_save_progress: bool,
    pub open_last_session_at_startup: bool,
    pub confrim_card_delete: bool, 
    pub confrim_progress_reset: bool, 
    pub animation_speed: f32,
    pub path_to_db_core: String,
    pub path_to_localization: String,
    pub path_to_kanji_localization: String,

    #[serde(default)]
    pub custom_decks: HashMap<String, Vec<i32>>,
}

impl Config {
    pub fn default() -> Self {
       Self {
            interface_font_size: 14.0,
            kanji_font_size: 48.0,
            auto_save_progress: false,
            open_last_session_at_startup: false,
            confrim_card_delete: false,
            confrim_progress_reset: false,
            animation_speed: 0.75,
            path_to_db_core: "db/core.db".to_string(),
            path_to_localization: "localization/en.json".to_string(),
            path_to_kanji_localization: "kanji_localization/en.json".to_string(),
            custom_decks: HashMap::new(),
        }
    }
}


// Load config
pub fn load_config<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let toml_str: T = toml::from_str(&contents)?;
    Ok(toml_str)
}

// Save config
pub fn save_config<T: Serialize>(path: &str, data: &T) -> anyhow::Result<()> {
    let toml_str = toml::to_string(data)?;
    let mut file = File::create(path)?;
    file.write_all(toml_str.as_bytes())?;
    Ok(())
}