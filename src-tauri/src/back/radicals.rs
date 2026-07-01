use crate::back::core::Kanji;
use roxmltree::{Document, ParsingOptions};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

/// Lazily parses each kanji's KanjiVG SVG for its top-level component
/// breakdown (e.g. 語 -> [言, 吾]) and caches the result by kanji id.
/// Mirrors `SvgCache`'s lazy-load-by-id pattern, kept separate since most
/// requests only need one of the two (stroke animation vs. this).
pub struct RadicalCache {
    data: HashMap<i32, Vec<String>>,
    svg_path: String,
    unicode_map: HashMap<i32, String>,
}

impl RadicalCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            svg_path: String::new(),
            unicode_map: HashMap::new(),
        }
    }

    pub fn prepare(&mut self, kanji_db: &[Arc<Kanji>], svg_path: &str) {
        self.svg_path = svg_path.to_string();
        self.unicode_map.clear();
        for k in kanji_db {
            self.unicode_map.insert(k.id, k.unicode.clone());
        }
    }

    /// Returns the kanji's direct child components, if its SVG decomposes
    /// into any (atomic radicals like 木 have none — an empty result).
    pub fn get_or_load(&mut self, kanji_id: i32) -> Option<&Vec<String>> {
        if !self.data.contains_key(&kanji_id) {
            if let Some(unicode) = self.unicode_map.get(&kanji_id).cloned() {
                let file_path = format!("{}/0{}.svg", self.svg_path, unicode.to_lowercase());
                if let Ok(content) = fs::read_to_string(&file_path) {
                    self.data.insert(kanji_id, parse_components(&content));
                }
            }
        }
        self.data.get(&kanji_id)
    }
}

/// KanjiVG wraps each kanji's strokes as `<g id="kvg:StrokePaths_...">`
/// (no `kvg:element`) containing a single `<g kvg:element="{char}">` for
/// the whole kanji. That inner group's direct `<g>` children (if any) are
/// its top-level components — e.g. for 語 that's 言 and 吾, each of which
/// may nest further sub-components we intentionally don't recurse into,
/// since "made of X + Y" is the useful depth for a learner. Found by
/// structure (outermost `kvg:element`-bearing group) rather than by
/// reconstructing its id, since that would depend on the hex-digit
/// zero-padding convention used for the file's own name.
fn parse_components(raw_text: &str) -> Vec<String> {
    let text = raw_text.replace("kvg:", "kvg_");
    let opt = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    let Ok(doc) = Document::parse_with_options(&text, opt) else {
        return Vec::new();
    };

    let root = doc.descendants().find(|n| {
        n.has_tag_name("g")
            && n.attribute("kvg_element").is_some()
            && n.parent().is_none_or(|p| p.attribute("kvg_element").is_none())
    });
    let Some(root) = root else {
        return Vec::new();
    };

    root.children()
        .filter(|c| c.has_tag_name("g"))
        .filter_map(|c| c.attribute("kvg_element"))
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decomposes_compound_kanji() {
        let content = std::fs::read_to_string("../data/kanji-svg/08a9e.svg").unwrap();
        assert_eq!(parse_components(&content), vec!["言", "吾"]);
    }

    #[test]
    fn atomic_radical_has_no_components() {
        let content = std::fs::read_to_string("../data/kanji-svg/06728.svg").unwrap();
        assert_eq!(parse_components(&content), Vec::<String>::new());
    }
}
