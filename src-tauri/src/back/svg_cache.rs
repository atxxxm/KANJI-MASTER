use crate::back::core::Kanji;
use kurbo::{BezPath, PathEl, Point};
use roxmltree::{Document, ParsingOptions};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use svgtypes::{PathParser, PathSegment};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct StrokePoint {
    pub pos: Point,
}

#[derive(Clone, Debug)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
}

pub struct SvgCache {
    pub data: HashMap<i32, Vec<Stroke>>,
}

impl SvgCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn load_all(&mut self, kanji_db: &[Arc<Kanji>], svg_path: &str) {
        for k in kanji_db {
            let file_path = format!("{}/0{}.svg", svg_path, k.unicode.to_lowercase());
            if Path::new(&file_path).exists()
                && let Ok(content) = fs::read_to_string(&file_path)
                    && let Some(strokes) = parse_svg_content(&content) {
                        self.data.insert(k.id, strokes);
                    }
        }
    }
}

fn parse_svg_content(raw_text: &str) -> Option<Vec<Stroke>> {
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