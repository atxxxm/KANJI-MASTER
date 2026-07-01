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
    data: HashMap<i32, Vec<Stroke>>,
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

    /// Returns strokes for a kanji, parsing its SVG file on first access.
    pub fn get_or_load(&mut self, kanji_id: i32) -> Option<&Vec<Stroke>> {
        if !self.data.contains_key(&kanji_id) {
            if let Some(unicode) = self.unicode_map.get(&kanji_id).cloned() {
                let file_path = format!("{}/0{}.svg", self.svg_path, unicode.to_lowercase());
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Some(strokes) = parse_svg_content(&content) {
                        self.data.insert(kanji_id, strokes);
                    }
                }
            }
        }
        self.data.get(&kanji_id)
    }
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