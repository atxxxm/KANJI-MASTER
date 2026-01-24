use rand::seq::SliceRandom;
use rand::rng;
use crate::back::core::Kanji;

// Active cards session status
pub struct CardsSession {
    pub queue: Vec<Kanji>, // Queue of kanji
    pub current_index: usize, // Current index of kanji in queue
    pub is_card_flipped: bool, // Is card flipped
    pub total_count: usize, // Total count of kanji
    pub finished: bool, // Is session finished
}

impl CardsSession {
    pub fn new(mut kanji_list: Vec<Kanji>, limit: usize) -> Self {
        let mut rng = rng();
        kanji_list.shuffle(&mut rng);

        // Cut to the limit (if the limit is 0, take everything)
        if limit > 0 && limit < kanji_list.len() {
            kanji_list.truncate(limit);
        }

        Self {
            total_count: kanji_list.len(),
            queue: kanji_list,
            current_index: 0,
            is_card_flipped: false,
            finished: false,
        }
    }

    pub fn next(&mut self) {
        if self.current_index < self.queue.len() - 1 {
            self.current_index += 1;
            self.is_card_flipped = false;
        } else {
            self.finished = true;
        }
    }
}

// Deck Builder Status
pub struct DeckBuilderState {
    pub deck_name_buffer: String,
    pub search_buffer: String,
    pub selected_kanji_idx: Vec<i32>,
    pub is_editing: bool,
}

impl Default for DeckBuilderState {
    fn default() -> Self {
        Self {
            deck_name_buffer: String::new(),
            search_buffer: String::new(),
            selected_kanji_idx: Vec::new(),
            is_editing: false,
        }
    }
}