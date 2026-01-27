use crate::back::core::Kanji;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub struct MessageTranslationData {
    pub file_load_success: String,
    pub file_not_found_or_invalid: String,
    pub saved_at_id: String,
    pub error_searialize_json: String,
    pub error_create_file: String,
}

impl Default for MessageTranslationData {
    fn default() -> Self {
        Self {
            file_load_success: "File loaded successfully".to_string(),
            file_not_found_or_invalid: "File not found or invalid".to_string(),
            saved_at_id: "Saved at ID".to_string(),
            error_searialize_json: "Error serializing JSON".to_string(),
            error_create_file: "Error creating file".to_string(),
        }
    }
}

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

pub struct TranslateState {
    path_to_file: String,                 // Path to JSON file
    message_data: MessageTranslationData, // Messages
    pub data: TranslationFile,            // All loaded data
    pub current_index: usize,             // Current index at vector self.kanji
    pub meaning_buffer: String,           // Buffer for meaning
    pub examples_buffer: Vec<String>,     // Buffer for examples
    pub status_message: String,           // Status message
}

impl Default for TranslateState {
    fn default() -> Self {
        Self {
            path_to_file: String::new(),
            message_data: MessageTranslationData::default(),
            data: TranslationFile::default(),
            current_index: 0,
            meaning_buffer: String::new(),
            examples_buffer: Vec::new(),
            status_message: String::new(),
        }
    }
}

impl TranslateState {
    pub fn new(path: &str, message_data: MessageTranslationData) -> Self {
        Self {
            path_to_file: path.to_string(),
            message_data,
            ..Default::default()
        }
    }

    // Load JSON from file
    pub fn load(&mut self, new_path: &str) -> bool {
        self.path_to_file = new_path.to_string();

        if let Ok(content) = fs::read_to_string(&self.path_to_file) {
            if let Ok(loaded_data) = serde_json::from_str::<TranslationFile>(&content) {
                self.data = loaded_data;
                self.status_message = self.message_data.file_load_success.clone();
                return true;
            }
        }
        // If file doesn't exist or error reading, data remains empty (or default)
        self.status_message = self.message_data.file_not_found_or_invalid.clone();
        false
    }

    // Auxiliary function to fill text fields from memory
    pub fn sync_translation_buffers(&mut self, kanji: &Kanji) {
        self.meaning_buffer.clear();
        self.examples_buffer.clear();

        // If there are saved data for this kanji
        if let Some(entry) = self.data.entries.get(&kanji.kanji) {
            self.meaning_buffer = entry.meaning.clone();
            self.examples_buffer = entry.translate_examples.clone();
        }

        // If count of examples in DB is more than in translation, resize buffer
        let target_len = kanji.example.len();
        if self.examples_buffer.len() < target_len {
            self.examples_buffer.resize(target_len, String::new());
        }
    }

    // Function for saving JSON
    pub fn save_translations(&mut self, kanji: &Vec<Kanji>) {
        // Update current entry before saving
        if let Some(k) = kanji.get(self.current_index) {
            let entry = KanjiTranslation {
                meaning: self.meaning_buffer.clone(),
                translate_examples: self.examples_buffer.clone(),
            };
            self.data.entries.insert(k.kanji.clone(), entry);
            self.data.last_id = k.id;
        }

        // Write to disk
        match fs::File::create(&self.path_to_file) {
            Ok(file) => {
                if serde_json::to_writer_pretty(file, &self.data).is_ok() {
                    self.status_message =
                        format!("{}: {}", self.message_data.saved_at_id, self.data.last_id);
                } else {
                    self.status_message = self.message_data.error_searialize_json.clone();
                }
            }
            Err(e) => {
                self.status_message = format!("{}: {}", &self.message_data.error_create_file, e);
            }
        }
    }
}
