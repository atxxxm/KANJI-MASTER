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

/// Substring search across the word itself, its reading, and both gloss
/// languages. Exact word/reading matches first, then frequency rank, then
/// shorter words — so "go" surfaces 語 compounds before long phrases.
pub fn search_words(db_path: &str, query: &str, limit: u32) -> Vec<Word> {
    if query.is_empty() {
        return Vec::new();
    }
    let Ok(conn) = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY) else {
        return Vec::new();
    };
    let Ok(mut stmt) = conn.prepare(
        "SELECT id, kanji, reading, gloss_en, gloss_ru, rank FROM words
         WHERE kanji LIKE '%' || ?1 || '%'
            OR reading LIKE '%' || ?1 || '%'
            OR gloss_en LIKE '%' || ?1 || '%'
            OR gloss_ru LIKE '%' || ?1 || '%'
         ORDER BY (kanji = ?1 OR reading = ?1) DESC,
                  rank ASC,
                  LENGTH(reading) ASC
         LIMIT ?2",
    ) else {
        return Vec::new();
    };
    stmt.query_map(rusqlite::params![query, limit], row_to_word)
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

/// Common words containing the given kanji character, most frequent first.
pub fn words_for_kanji(db_path: &str, ch: &str, limit: u32) -> Vec<Word> {
    let Ok(conn) = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY) else {
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
