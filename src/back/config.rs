use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub interface_font_size: f32,
    pub kanji_font_size: f32,
    pub animation_speed: f32,
    pub path_to_db_core: String,
    pub path_to_localization: String,
    pub path_to_kanji_localization: String,
    pub show_kanji_meaning: bool,
    pub focus_on_search: bool,

    #[serde(default)]
    pub custom_decks: HashMap<String, Vec<i32>>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            interface_font_size: 14.0,
            kanji_font_size: 48.0,
            animation_speed: 0.75,
            path_to_db_core: "db/core.db".to_string(),
            path_to_localization: "localization/en.json".to_string(),
            path_to_kanji_localization: "kanji_localization/en.json".to_string(),
            show_kanji_meaning: false,
            focus_on_search: true,
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
