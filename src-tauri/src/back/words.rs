use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;

/// One JMdict entry from the bundled `words` table (common-priority words
/// only). `kanji` is None for kana-only words; glosses may be missing in
/// either language.
#[derive(Serialize, Clone)]
pub struct Word {
    pub id: i32,
    pub kanji: Option<String>,
    pub reading: String,
    pub gloss_en: Option<String>,
    pub gloss_ru: Option<String>,
    pub rank: i32,
}

fn row_to_word(row: &rusqlite::Row) -> rusqlite::Result<Word> {
    Ok(Word {
        id: row.get("id")?,
        kanji: row.get("kanji")?,
        reading: row.get("reading")?,
        gloss_en: row.get("gloss_en")?,
        gloss_ru: row.get("gloss_ru")?,
        rank: row.get("rank")?,
    })
}

/// Opens the DB read-only and registers `ulower`, a Unicode-aware lowercase
/// helper. SQLite's built-in `lower()`/`LIKE` only case-fold ASCII, so
/// Cyrillic (and other non-ASCII) gloss searches would otherwise be
/// case-sensitive.
fn open(db_path: &str) -> Option<Connection> {
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).ok()?;
    conn.create_scalar_function(
        "ulower",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let s: Option<String> = ctx.get(0)?;
            Ok(s.map(|s| s.to_lowercase()))
        },
    )
    .ok()?;
    Some(conn)
}

/// Substring search across the word itself, its reading, and both gloss
/// languages. Case-insensitive for all scripts (via `ulower`). Exact
/// word/reading matches first, then frequency rank, then shorter words —
/// so "go" surfaces 語 compounds before long phrases.
pub fn search_words(db_path: &str, query: &str, limit: u32) -> Vec<Word> {
    if query.is_empty() {
        return Vec::new();
    }
    let Some(conn) = open(db_path) else {
        return Vec::new();
    };
    let q = query.to_lowercase();
    let Ok(mut stmt) = conn.prepare(
        "SELECT id, kanji, reading, gloss_en, gloss_ru, rank FROM words
         WHERE kanji LIKE '%' || ?1 || '%'
            OR reading LIKE '%' || ?1 || '%'
            OR ulower(gloss_en) LIKE '%' || ?1 || '%'
            OR ulower(gloss_ru) LIKE '%' || ?1 || '%'
         ORDER BY (kanji = ?1 OR reading = ?1) DESC,
                  rank ASC,
                  LENGTH(reading) ASC
         LIMIT ?2",
    ) else {
        return Vec::new();
    };
    stmt.query_map(rusqlite::params![q, limit], row_to_word)
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

/// Common words containing the given kanji character, most frequent first.
pub fn words_for_kanji(db_path: &str, ch: &str, limit: u32) -> Vec<Word> {
    let Some(conn) = open(db_path) else {
        return Vec::new();
    };
    let Ok(mut stmt) = conn.prepare(
        "SELECT id, kanji, reading, gloss_en, gloss_ru, rank FROM words
         WHERE kanji LIKE '%' || ?1 || '%'
         ORDER BY rank ASC, LENGTH(kanji) ASC
         LIMIT ?2",
    ) else {
        return Vec::new();
    };
    stmt.query_map(rusqlite::params![ch, limit], row_to_word)
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

/// Fetches a single word by its row id, for the word detail page.
pub fn get_word(db_path: &str, id: i32) -> Option<Word> {
    let conn = open(db_path)?;
    let mut stmt = conn
        .prepare("SELECT id, kanji, reading, gloss_en, gloss_ru, rank FROM words WHERE id = ?1")
        .ok()?;
    stmt.query_row(rusqlite::params![id], row_to_word).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Builds a throwaway SQLite file (rusqlite's read-only OpenFlags used by
    /// `open()` require a real file — ":memory:" won't work since each
    /// connection to it is an isolated database) with a `words` table
    /// seeded with a few realistic rows, and returns its path. The caller's
    /// writable `Connection` is dropped before returning so the read-only
    /// `open()` calls under test don't contend with it.
    fn seeded_db() -> String {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("kanji_master_words_test_{nonce}.db"));
        let path_str = path.to_string_lossy().into_owned();

        let conn = Connection::open(&path_str).unwrap();
        conn.execute_batch(
            "CREATE TABLE words (
                id INTEGER PRIMARY KEY,
                kanji TEXT,
                reading TEXT NOT NULL,
                gloss_en TEXT,
                gloss_ru TEXT,
                rank INTEGER
            );
            INSERT INTO words (id, kanji, reading, gloss_en, gloss_ru, rank) VALUES
                (1, '日本語', 'にほんご', 'Japanese language', 'японский язык', 10),
                (2, '語', 'ご', 'language; word', 'язык; слово', 50),
                (3, '英語', 'えいご', 'English language', 'английский язык', 20),
                (4, NULL, 'こんにちは', 'hello', 'привет', 5);",
        )
        .unwrap();
        drop(conn);

        path_str
    }

    #[test]
    fn search_matches_kanji_reading_and_english_gloss() {
        let db = seeded_db();
        assert_eq!(search_words(&db, "語", 10).len(), 3); // 日本語, 語, 英語
        assert_eq!(search_words(&db, "にほん", 10).len(), 1);
        assert_eq!(search_words(&db, "hello", 10).len(), 1);
    }

    #[test]
    fn search_is_case_insensitive_for_cyrillic_glosses() {
        // This is the exact bug fixed by the ulower() SQLite function: plain
        // LIKE only case-folds ASCII, so uppercase/mixed-case Cyrillic input
        // would otherwise miss lowercase-stored glosses.
        let db = seeded_db();
        assert_eq!(search_words(&db, "ЯЗЫК", 10).len(), 3);
        assert_eq!(search_words(&db, "Привет", 10).len(), 1);
        assert_eq!(search_words(&db, "язык", 10).len(), 3);
    }

    #[test]
    fn search_is_case_insensitive_for_english_glosses_too() {
        let db = seeded_db();
        assert_eq!(search_words(&db, "HELLO", 10).len(), 1);
        assert_eq!(search_words(&db, "Japanese", 10).len(), 1);
    }

    #[test]
    fn exact_match_and_frequency_rank_order_results() {
        let db = seeded_db();
        let results = search_words(&db, "語", 10);
        // Exact reading match ("ご", rank 50) still loses to non-exact but
        // lower-rank (more frequent) matches per the ORDER BY: exact-match
        // flag is checked first, but among the non-exact matches rank wins.
        // 日本語 (rank 10) should come before 英語 (rank 20).
        let kanji: Vec<_> = results.iter().map(|w| w.kanji.clone()).collect();
        let pos_nihongo = kanji.iter().position(|k| k.as_deref() == Some("日本語")).unwrap();
        let pos_eigo = kanji.iter().position(|k| k.as_deref() == Some("英語")).unwrap();
        assert!(pos_nihongo < pos_eigo, "more frequent word should sort first");
    }

    #[test]
    fn search_respects_the_limit() {
        let db = seeded_db();
        assert_eq!(search_words(&db, "語", 1).len(), 1);
    }

    #[test]
    fn empty_query_returns_nothing() {
        let db = seeded_db();
        assert!(search_words(&db, "", 10).is_empty());
    }

    #[test]
    fn words_for_kanji_finds_only_matching_entries_most_frequent_first() {
        let db = seeded_db();
        let results = words_for_kanji(&db, "語", 10);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].kanji.as_deref(), Some("日本語")); // rank 10, most frequent

        // A kana-only word (no kanji field) never matches a kanji search.
        let results = words_for_kanji(&db, "今", 10);
        assert!(results.is_empty());
    }

    #[test]
    fn get_word_returns_the_matching_row_or_none() {
        let db = seeded_db();
        let word = get_word(&db, 1).expect("id 1 should exist");
        assert_eq!(word.kanji.as_deref(), Some("日本語"));
        assert_eq!(word.gloss_ru.as_deref(), Some("японский язык"));

        assert!(get_word(&db, 9999).is_none());
    }

    #[test]
    fn missing_database_file_returns_empty_results_not_a_panic() {
        assert!(search_words("Z:/definitely/does/not/exist.db", "test", 10).is_empty());
        assert!(words_for_kanji("Z:/definitely/does/not/exist.db", "語", 10).is_empty());
        assert!(get_word("Z:/definitely/does/not/exist.db", 1).is_none());
    }
}
