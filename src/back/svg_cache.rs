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
    pub dist: f32,
}

#[derive(Clone, Debug)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub total_length: f32,
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
            if Path::new(&file_path).exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Some(strokes) = parse_svg_content(&content) {
                        self.data.insert(k.id, strokes);
                    }
                }
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
        if node.has_tag_name("path") {
            if let Some(d) = node.attribute("d") {
                let mut bez_path = BezPath::new();
                let mut current = Point::ZERO;

                for seg in PathParser::from(d) {
                    if let Ok(segment) = seg {
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
                }

                let mut points = Vec::new();
                let mut total_dist = 0.0;
                let mut is_first = true;
                let mut last_pos = Point::ZERO;

                kurbo::flatten(bez_path.iter(), 0.2, |el| match el {
                    PathEl::MoveTo(p) => {
                        last_pos = p;
                        if is_first {
                            points.push(StrokePoint { pos: p, dist: 0.0 });
                            is_first = false;
                        }
                    }
                    PathEl::LineTo(p) => {
                        let dist = last_pos.distance(p);
                        total_dist += dist;
                        points.push(StrokePoint { pos: p, dist: total_dist as f32 });
                        last_pos = p;
                    }
                    _ => {}
                });

                if !points.is_empty() {
                    strokes.push(Stroke {
                        points,
                        total_length: total_dist as f32,
                    });
                }
            }
        }
    }
    
    if strokes.is_empty() { None } else { Some(strokes) }
}