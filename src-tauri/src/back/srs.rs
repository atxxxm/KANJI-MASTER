use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Anki-style answer ratings. Sent from the frontend as 0..=3.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rating {
    Again,
    Hard,
    Good,
    Easy,
}

impl Rating {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Rating::Again),
            1 => Some(Rating::Hard),
            2 => Some(Rating::Good),
            3 => Some(Rating::Easy),
            _ => None,
        }
    }
}

/// Per-kanji SM-2 scheduling state. Keyed by the kanji character (not the
/// DB id) so progress survives dictionary reorderings/deletions.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardState {
    pub ease: f32,
    pub interval_days: f32,
    /// Epoch day (UTC) when this card is next due.
    pub due_day: i64,
    pub reps: u32,
    pub lapses: u32,
    /// True while the card is in the learning/relearning phase; such cards
    /// repeat within the same session until answered Good/Easy.
    pub learning: bool,
}

impl CardState {
    fn new(today: i64) -> Self {
        Self {
            ease: 2.5,
            interval_days: 0.0,
            due_day: today,
            reps: 0,
            lapses: 0,
            learning: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SrsSettings {
    /// JLPT levels new cards are introduced from, e.g. ["N5", "N4"].
    pub levels: Vec<String>,
    pub new_per_day: u32,
}

impl Default for SrsSettings {
    fn default() -> Self {
        Self {
            levels: vec!["N5".to_string()],
            new_per_day: 10,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SrsState {
    #[serde(default)]
    pub settings: SrsSettings,
    #[serde(default)]
    pub cards: HashMap<String, CardState>,
    /// Epoch day the daily new-card counter was last reset on.
    #[serde(default)]
    pub last_study_day: i64,
    #[serde(default)]
    pub new_introduced_today: u32,
}

pub fn today_epoch_day() -> i64 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    (secs / 86_400) as i64
}

impl SrsState {
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    /// Resets the daily new-card counter when the day rolls over.
    pub fn roll_day(&mut self, today: i64) {
        if self.last_study_day != today {
            self.last_study_day = today;
            self.new_introduced_today = 0;
        }
    }

    pub fn new_remaining_today(&self) -> u32 {
        self.settings.new_per_day.saturating_sub(self.new_introduced_today)
    }

    /// Applies an SM-2 answer to the card (creating it on first contact).
    /// Returns true when the card should repeat within the current session.
    pub fn answer(&mut self, kanji: &str, rating: Rating, today: i64) -> bool {
        self.roll_day(today);

        let is_new = !self.cards.contains_key(kanji);
        let card = self
            .cards
            .entry(kanji.to_string())
            .or_insert_with(|| CardState::new(today));
        if is_new {
            self.new_introduced_today += 1;
        }

        apply_rating(card, rating, today)
    }

    /// Predicted next-review intervals (in days) for each of the four ratings,
    /// without mutating anything — used to label the answer buttons. A value
    /// of 0 means the card repeats later in the same session (Again / still
    /// learning), i.e. under a day.
    pub fn preview(&self, kanji: &str, today: i64) -> [f32; 4] {
        let base = self
            .cards
            .get(kanji)
            .cloned()
            .unwrap_or_else(|| CardState::new(today));
        let mut out = [0.0f32; 4];
        for (i, rating) in [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy]
            .into_iter()
            .enumerate()
        {
            let mut card = base.clone();
            let requeue = apply_rating(&mut card, rating, today);
            out[i] = if requeue { 0.0 } else { (card.due_day - today) as f32 };
        }
        out
    }

    /// Kanji chars due for review today, learning cards first, then oldest due.
    pub fn due_cards(&self, today: i64) -> Vec<String> {
        let mut due: Vec<(&String, &CardState)> = self
            .cards
            .iter()
            .filter(|(_, c)| c.due_day <= today)
            .collect();
        due.sort_by_key(|(_, c)| (!c.learning, c.due_day));
        due.into_iter().map(|(k, _)| k.clone()).collect()
    }

    /// Cards with a mature interval (Anki's 21-day convention).
    pub fn mature_count(&self) -> usize {
        self.cards.values().filter(|c| !c.learning && c.interval_days >= 21.0).count()
    }
}

/// Core SM-2 state transition, shared by `answer` (mutates the stored card)
/// and `preview` (runs it on a throwaway clone). Returns true when the card
/// should be shown again in the same session.
fn apply_rating(card: &mut CardState, rating: Rating, today: i64) -> bool {
    card.reps += 1;

    if card.learning {
        match rating {
            Rating::Again | Rating::Hard => {
                // Stay in learning; repeat this session.
                card.due_day = today;
                return true;
            }
            Rating::Good => {
                card.learning = false;
                card.interval_days = 1.0;
            }
            Rating::Easy => {
                card.learning = false;
                card.interval_days = 4.0;
            }
        }
    } else {
        match rating {
            Rating::Again => {
                card.lapses += 1;
                card.ease = (card.ease - 0.2).max(1.3);
                card.learning = true;
                card.interval_days = 0.0;
                card.due_day = today;
                return true;
            }
            Rating::Hard => {
                card.ease = (card.ease - 0.15).max(1.3);
                card.interval_days = (card.interval_days * 1.2).max(card.interval_days + 1.0);
            }
            Rating::Good => {
                card.interval_days *= card.ease;
            }
            Rating::Easy => {
                card.interval_days *= card.ease * 1.3;
                card.ease += 0.15;
            }
        }
    }

    card.due_day = today + (card.interval_days.round() as i64).max(1);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = 20_000;

    #[test]
    fn new_card_graduates_with_good() {
        let mut s = SrsState::default();
        let requeue = s.answer("水", Rating::Good, DAY);
        assert!(!requeue);
        let c = &s.cards["水"];
        assert!(!c.learning);
        assert_eq!(c.due_day, DAY + 1);
    }

    #[test]
    fn again_keeps_card_in_session() {
        let mut s = SrsState::default();
        let requeue = s.answer("水", Rating::Again, DAY);
        assert!(requeue);
        let c = &s.cards["水"];
        assert!(c.learning);
        assert_eq!(c.due_day, DAY);
    }

    #[test]
    fn review_good_multiplies_interval_by_ease() {
        let mut s = SrsState::default();
        s.answer("水", Rating::Good, DAY); // graduate: interval 1
        let requeue = s.answer("水", Rating::Good, DAY + 1);
        assert!(!requeue);
        let c = &s.cards["水"];
        // 1.0 * 2.5 = 2.5 -> rounds to 3 (min 1) days out from DAY+1
        assert_eq!(c.due_day, DAY + 1 + 3);
    }

    #[test]
    fn lapse_resets_to_learning_and_lowers_ease() {
        let mut s = SrsState::default();
        s.answer("水", Rating::Good, DAY);
        let requeue = s.answer("水", Rating::Again, DAY + 1);
        assert!(requeue);
        let c = &s.cards["水"];
        assert!(c.learning);
        assert_eq!(c.lapses, 1);
        assert!(c.ease < 2.5);
    }

    #[test]
    fn daily_new_counter_rolls_over() {
        let mut s = SrsState::default();
        s.answer("水", Rating::Good, DAY);
        s.answer("火", Rating::Good, DAY);
        assert_eq!(s.new_introduced_today, 2);
        s.roll_day(DAY + 1);
        assert_eq!(s.new_introduced_today, 0);
    }

    #[test]
    fn due_cards_orders_learning_first() {
        let mut s = SrsState::default();
        s.answer("水", Rating::Good, DAY - 5); // review card, due DAY-4
        s.answer("火", Rating::Again, DAY);    // learning card, due DAY
        let due = s.due_cards(DAY);
        assert_eq!(due, vec!["火".to_string(), "水".to_string()]);
    }

    #[test]
    fn preview_matches_actual_answer_and_does_not_mutate() {
        let mut s = SrsState::default();
        let p = s.preview("水", DAY); // brand-new card
        // Again/Hard requeue this session (0); Good graduates to 1d, Easy 4d.
        assert_eq!(p, [0.0, 0.0, 1.0, 4.0]);
        // Preview must not have created or altered any card state.
        assert!(s.cards.is_empty());

        // The Good preview should equal what actually answering Good does.
        let requeue = s.answer("水", Rating::Good, DAY);
        assert!(!requeue);
        assert_eq!(s.cards["水"].due_day - DAY, p[2] as i64);
    }
}
