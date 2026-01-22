use std::fs::File;

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use anyhow;
// Main Localization Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Localization {
    pub lang: String,
    pub local: Local,
    
}

// Local Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Local {
    pub top_bar: TopBar,
    pub settings: Settings,
    pub screens: Screens,
}

// Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TopBar {
    pub kanji: KanjiTopBar,
    pub tranning: TranningTopBar,
    pub card: CardTopBar,
    pub kana: KanaTopBar,
}

// Kanji Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanjiTopBar {
    pub title: String,
    pub jlpt: String,
    pub kanaken: String,
    pub radicals: String,
    pub all: String,
}

// Tranning Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TranningTopBar {
    pub title: String,
    pub jlpt: String,
    pub kanaken: String,
    pub custom: String,
}

// Card Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CardTopBar {
    pub title: String,
    pub new: String,
    pub open: String,
}

// Kana Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanaTopBar {
    pub title: String,
    pub hiragana: String,
    pub katakana: String,
}

// Settings Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub title: String,
    pub lang: String,
    pub lang_button: String,
    pub interface_font_size: String,
    pub kanji_font_size: String,
    pub auto_save_progress: String,
    pub auto_save_frequency: String,
    pub open_last_session_at_startup: String,
    pub startup_screen: String,
    pub confrim_card_delete: String,
    pub confrim_progress_reset: String,
}

// Screens Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Screens {
    pub search: String,
    pub current_kanji: CurrentKanji,
}

// Current Kanji Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CurrentKanji {
    pub back_button: String,
    pub information: String,
    pub meaning: String,
    pub onyomi: String,
    pub onyomi_romaji: String,
    pub kunyomi: String,
    pub kunyomi_romaji: String,
    pub strokes: String,
    pub jlpt: String,
    pub grade: String,
    pub frequency: String,
    pub examples: String,
}

// Save file (Serialize)
pub fn save<T: Serialize>(path: &str, data: &T) -> anyhow::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

// Load file (Deserialize)
pub fn load<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let file = File::open(path)?;
    let data: T = serde_json::from_reader(file)?;
    Ok(data)
}