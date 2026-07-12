use crate::back::core::Kanji;
use kurbo::{BezPath, PathEl, Point};
use roxmltree::{Document, ParsingOptions};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use svgtypes::{PathParser, PathSegment};

#[derive(Clone, Debug)]
pub struct StrokePoint {
    pub pos: Point,
}

#[derive(Clone, Debug)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
}

pub struct SvgCache {
    // Each kanji's stroke paths as their raw SVG `d` attribute strings, in
    // drawing order. The frontend renders these directly as <path> elements
    // and animates them with stroke-dashoffset, so no flattening to points is
    // needed here (unlike `parse_svg_content`, which recognition still uses).
    data: HashMap<i32, Vec<String>>,
    svg_path: String,
    unicode_map: HashMap<i32, String>,
}

impl SvgCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            svg_path: String::new(),
            unicode_map: HashMap::new(),
        }
    }

    /// Stores the path and kanji→unicode mapping; does NOT parse any SVG files.
    pub fn prepare(&mut self, kanji_db: &[Arc<Kanji>], svg_path: &str) {
        self.svg_path = svg_path.to_string();
        self.unicode_map.clear();
        for k in kanji_db {
            self.unicode_map.insert(k.id, k.unicode.clone());
        }
    }

    /// Returns each stroke's SVG path data for a kanji, parsing its SVG file
    /// on first access and caching the result.
    pub fn get_or_load(&mut self, kanji_id: i32) -> Option<&Vec<String>> {
        if !self.data.contains_key(&kanji_id) {
            if let Some(unicode) = self.unicode_map.get(&kanji_id).cloned() {
                let file_path = format!("{}/0{}.svg", self.svg_path, unicode.to_lowercase());
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Some(paths) = parse_svg_paths(&content) {
                        self.data.insert(kanji_id, paths);
                    }
                }
            }
        }
        self.data.get(&kanji_id)
    }
}

/// Extracts each stroke's raw `d` path string in document (drawing) order.
/// KanjiVG puts stroke-number labels in `<text>` elements, so filtering to
/// `<path>` yields exactly the strokes — the same filter `parse_svg_content`
/// relies on.
pub(crate) fn parse_svg_paths(raw_text: &str) -> Option<Vec<String>> {
    let text = raw_text.replace("kvg:", "kvg_");
    let opt = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    let doc = Document::parse_with_options(&text, opt).ok()?;

    let paths: Vec<String> = doc
        .descendants()
        .filter(|n| n.has_tag_name("path"))
        .filter_map(|n| n.attribute("d").map(String::from))
        .collect();

    if paths.is_empty() { None } else { Some(paths) }
}

pub(crate) fn parse_svg_content(raw_text: &str) -> Option<Vec<Stroke>> {
    let text = raw_text.replace("kvg:", "kvg_");
    let opt = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    
    let doc = Document::parse_with_options(&text, opt).ok()?;
    let mut strokes = Vec::new();

    for node in doc.descendants() {
        if node.has_tag_name("path")
            && let Some(d) = node.attribute("d") {
                let mut bez_path = BezPath::new();
                let mut current = Point::ZERO;

                for segment in PathParser::from(d).flatten() {
                    match segment {
                        PathSegment::MoveTo { abs, x, y } => {
                            let p = if abs { Point::new(x, y) } else { current + (x, y) };
                            bez_path.move_to(p);
                            current = p;
                        }
                        PathSegment::LineTo { abs, x, y } => {
                            let p = if abs { Point::new(x, y) } else { current + (x, y) };
                            bez_path.line_to(p);
                            current = p;
                        }
                        PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y } => {
                            let c1 = if abs { Point::new(x1, y1) } else { current + (x1, y1) };
                            let c2 = if abs { Point::new(x2, y2) } else { current + (x2, y2) };
                            let p = if abs { Point::new(x, y) } else { current + (x, y) };
                            bez_path.curve_to(c1, c2, p);
                            current = p;
                        }
                        _ => {}
                    }
                }

                let mut points = Vec::new();
                let mut is_first = true;

                kurbo::flatten(bez_path.iter(), 0.2, |el| match el {
                    PathEl::MoveTo(p) => {
                        if is_first {
                            points.push(StrokePoint { pos: p });
                            is_first = false;
                        }
                    }
                    PathEl::LineTo(p) => {
                        points.push(StrokePoint { pos: p });
                    }
                    _ => {}
                });

                if !points.is_empty() {
                    strokes.push(Stroke { points });
                }
            }
    }
    
    if strokes.is_empty() { None } else { Some(strokes) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parses_stroke_count_matching_the_svg_path_elements() {
        // 木 (06728.svg) has exactly 4 <path> elements/strokes.
        let content = fs::read_to_string("../data/kanji-svg/06728.svg").unwrap();
        let strokes = parse_svg_content(&content).expect("木 should parse to some strokes");
        assert_eq!(strokes.len(), 4);

        // 語 (08a9e.svg), a compound kanji, has 14.
        let content = fs::read_to_string("../data/kanji-svg/08a9e.svg").unwrap();
        let strokes = parse_svg_content(&content).expect("語 should parse to some strokes");
        assert_eq!(strokes.len(), 14);
    }

    #[test]
    fn every_stroke_has_at_least_two_points_to_draw_a_line() {
        let content = fs::read_to_string("../data/kanji-svg/06728.svg").unwrap();
        let strokes = parse_svg_content(&content).unwrap();
        for stroke in &strokes {
            assert!(stroke.points.len() >= 2, "a single-point stroke can't be animated as a line");
        }
    }

    #[test]
    fn malformed_or_pathless_svg_returns_none() {
        assert!(parse_svg_content("").is_none());
        assert!(parse_svg_content("<svg></svg>").is_none());
        assert!(parse_svg_content("not even xml").is_none());
    }

    #[test]
    fn parse_svg_paths_returns_one_d_string_per_stroke_in_order() {
        // Same stroke counts as parse_svg_content, but the raw `d` strings the
        // frontend animates directly instead of flattened points.
        let content = fs::read_to_string("../data/kanji-svg/06728.svg").unwrap();
        let paths = parse_svg_paths(&content).expect("木 should have stroke paths");
        assert_eq!(paths.len(), 4);
        assert!(paths.iter().all(|d| !d.trim().is_empty()));

        let content = fs::read_to_string("../data/kanji-svg/08a9e.svg").unwrap();
        assert_eq!(parse_svg_paths(&content).unwrap().len(), 14);
    }

    #[test]
    fn parse_svg_paths_on_pathless_input_returns_none() {
        assert!(parse_svg_paths("").is_none());
        assert!(parse_svg_paths("<svg></svg>").is_none());
    }
}