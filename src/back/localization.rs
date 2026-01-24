use std::fs::File;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use anyhow;

// Paths to files Struct
pub struct Paths {
    pub path_to_db_core: String,
    pub path_to_localization: String,
    pub path_to_kanji_localization: String,
}

// Main Localization Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Localization {
    pub lang: String,
    pub local: Local,
    
}

// Local Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Local {
    pub home: Home,
    pub top_bar: TopBar,
    pub settings: Settings,
    pub screens: Screens,
    pub kana: Kana,
}

// Home Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Home {
    pub kanji_search_hint: String,
    pub kanji_not_found: String,
}

// Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TopBar {
    pub kanji: KanjiTopBar,
    pub cards: CardsTopBar,
    pub kana: KanaTopBar,
    pub tools: Tools,
}

// Kanji Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanjiTopBar {
    pub title: String,
    pub all: String,
}

// Tranning Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CardsTopBar {
    pub title: String,
    pub card_setup_title: String,
    pub jlpt_level: String,
    pub card_count: String,
    pub card_setup_start: String,
    pub custom_deck_title: String,
    pub manage_decks: String,
    pub my_decks: String,
    pub no_custom_decks_created: String,
    pub delete_deck: String,
    pub edit_deck: String,
    pub play_deck: String,
    pub session_complete: String,
    pub return_to_menu: String,
    pub card: String,
    pub exit_training: String,
    pub onyomi: String,
    pub kunyomi: String,
    pub click_to_flip: String,
    pub next_card: String,
    pub back: String,
    pub deck_manager: String,
    pub deck_name: String,
    pub update_deck: String,
    pub save_deck: String,
    pub avaliable_kanji: String,
    pub search_kanji_hint: String,
    pub add_kanji: String,
    pub type_search_hint: String,
    pub deck_content: String,

}

// Kana Top Bar Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanaTopBar {
    pub title: String,
    pub hiragana: String,
    pub katakana: String,
}

// Tools Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tools {
    pub title: String,
    pub romaji_to_kana: String,
    pub translate_kanji: String,
    pub romaji_to_kana_locale: RomajiToKana,
    pub translate_kanji_locale: TranslateToKanji,
}

// Romaji To Kana Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RomajiToKana {
    pub title: String,
    pub output_mode: String,
    pub output_hiragana: String,
    pub output_katakana: String,
    pub input: String,
    pub hint_input: String,
    pub output: String,
    pub copy_button: String,
}

// Translate To Kanji Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TranslateToKanji {
    // Messages
    pub file_load_success: String,
    pub file_not_found_or_invalid: String,
    pub saved_at_id: String,
    pub error_searialize_json: String,
    pub error_create_file: String,
    pub found: String,
    pub not_found: String,

    // Screen
    pub title: String,
    pub jump_to: String,
    pub hint_kanji_input: String,
    pub go_button: String,
    pub save_progress_button: String,
    pub all_kanji_processed: String,
    pub meaning: String,
    pub hint_meaning_input: String,
    pub examples: String,
    pub hint_examples_input: String,
    pub error_buffer_mismatch: String,
    pub previous_button: String,
    pub next_button: String,


}

// Settings Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub title: String,
    pub lang: String,
    pub kanji_localization: String,
    pub lang_button: String,
    pub interface_font_size: String,
    pub kanji_animation_speed: String,
    pub kanji_font_size: String,
    pub auto_save_progress: String,
    pub auto_save_frequency: String,
    pub open_last_session_at_startup: String,
    pub startup_screen: String,
    pub confrim_card_delete: String,
    pub confrim_progress_reset: String,
    pub show_kanji_meaning: String,
    pub tools: String,
    pub create_default_localization_file: String,
    pub focus_on_search: String,
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

// Kana Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Kana {
    pub title: String,
    pub hiragana: String,
    pub katakana: String,
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