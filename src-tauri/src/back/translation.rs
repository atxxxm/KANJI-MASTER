use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Kanji Translation Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanjiTranslation {
    pub meaning: String,
    pub translate_examples: Vec<String>,
}

// Translation File Struct
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TranslationFile {
    pub last_id: i32, // ID last saved kanji
    pub entries: HashMap<String, KanjiTranslation>,
}
