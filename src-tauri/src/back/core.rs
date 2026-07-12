use anyhow;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Database {
    path_to_db: String,
}

#[derive(Clone, PartialEq)]
pub struct Kanji {
    pub id: i32,
    pub kanji: String,
    pub strokes: i8,
    pub jlpt: String,
    pub grade: String,
    pub frequency: String,
    pub unicode: String,
    pub onyomi: String,
    pub onyomi_romaji: String,
    pub kunyomi: String,
    pub kunyomi_romaji: String,
    pub example: Vec<String>,
}

impl Database {
    pub fn new(path_to_db: &str) -> Self {
        Database {
            path_to_db: path_to_db.to_string(),
        }
    }

    pub fn get_kanji(&self) -> anyhow::Result<Vec<Arc<Kanji>>> {
        // Read-only: the DB is shipped as a bundled resource and never written.
        // This also avoids silently creating an empty DB when the path is wrong.
        let conn = Connection::open_with_flags(
            &self.path_to_db,
            OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;

        let mut stmt = conn.prepare(
            "
            SELECT
                k.id,
                k.kanji,
                k.strokes,
                k.jlpt,
                k.grade,
                k.frequency,
                k.unicode,
                r.onyomi,
                r.onyomi_romaji,
                r.kunyomi,
                r.kunyomi_romaji
            FROM kanji k
            LEFT JOIN read r ON k.id = r.kanji_id
        ",
        )?;

        let mut kanji_map: HashMap<i32, Kanji> = HashMap::new();

        let rows = stmt.query_map([], |row| {
            let id: i32 = row.get("id")?;
            let kanji = Kanji {
                id,
                kanji: row.get("kanji")?,
                strokes: row.get("strokes")?,
                jlpt: row.get("jlpt")?,
                grade: row.get("grade")?,
                frequency: row.get("frequency")?,
                unicode: row.get("unicode")?,
                onyomi: row
                    .get::<_, Option<String>>("onyomi")?
                    .map_or(String::new(), |s| split_on_or_kun(&s)),
                onyomi_romaji: row
                    .get::<_, Option<String>>("onyomi_romaji")?
                    .map_or(String::new(), |s| split_romaji(&s)),
                kunyomi: row
                    .get::<_, Option<String>>("kunyomi")?
                    .map_or(String::new(), |s| split_on_or_kun(&s)),
                kunyomi_romaji: row
                    .get::<_, Option<String>>("kunyomi_romaji")?
                    .map_or(String::new(), |s| split_romaji(&s)),
                example: Vec::new(),
            };
            Ok((id, kanji))
        })?;

        for row in rows {
            let (id, k) = row?;
            kanji_map.insert(id, k);
        }

        let mut stmt_ex = conn.prepare("SELECT kanji_id, example FROM examples")?;
        let examples = stmt_ex.query_map([], |row| {
            Ok((
                row.get::<_, i32>("kanji_id")?,
                row.get::<_, String>("example")?,
            ))
        })?;

        for ex in examples {
            let (kanji_id, example_text) = ex?;
            if let Some(k) = kanji_map.get_mut(&kanji_id) {
                for part in example_text.split_whitespace() {
                    k.example.push(part.to_string());
                }
            }
        }

        let mut result: Vec<Kanji> = kanji_map.into_values().collect();
        result.sort_by(|a, b| a.kanji.cmp(&b.kanji));

        Ok(result.into_iter().map(Arc::new).collect())
    }

}

fn split_on_or_kun(text: &str) -> String {
    if text.trim().is_empty() {
        return String::new();
    }
    text.split_whitespace().collect::<Vec<_>>().join("、")
}

fn split_romaji(text: &str) -> String {
    if text.trim().is_empty() {
        return String::new();
    }
    text.split_whitespace().collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_or_kun_joins_with_japanese_comma() {
        assert_eq!(split_on_or_kun("カン ケン"), "カン、ケン");
        assert_eq!(split_on_or_kun("ヒト"), "ヒト");
    }

    #[test]
    fn split_on_or_kun_collapses_irregular_whitespace() {
        // KANJIDIC source fields are whitespace-separated with inconsistent
        // spacing/newlines — split_whitespace() must normalize that away.
        assert_eq!(split_on_or_kun("  カン   ケン  "), "カン、ケン");
    }

    #[test]
    fn split_on_or_kun_empty_or_blank_becomes_empty_string() {
        assert_eq!(split_on_or_kun(""), "");
        assert_eq!(split_on_or_kun("   "), "");
    }

    #[test]
    fn split_romaji_joins_with_comma_space() {
        assert_eq!(split_romaji("kan ken"), "kan, ken");
    }

    #[test]
    fn split_romaji_empty_or_blank_becomes_empty_string() {
        assert_eq!(split_romaji(""), "");
        assert_eq!(split_romaji("  "), "");
    }
}
