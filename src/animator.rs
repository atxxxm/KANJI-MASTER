use eframe::egui;
use std::fs;
use roxmltree::{Document, ParsingOptions};
use svgtypes::{PathParser, PathSegment};
use kurbo::{BezPath, PathEl, Point};
use kurbo::flatten;

pub struct KanjiAnimator {
    strokes: Vec<Vec<egui::Pos2>>, // List of strokes
    total_points: usize, // Total number of points in all strokes
    progress: usize, // Current progress through the animation
    is_playing: bool, // Flag to indicate if the animation is playing
    scale: f32, // Scale factor for the animation
    time_per_stroke: f32, // Time per stroke
    stroke_progress: f32, // Progress through the current stroke
    current_stroke_index: usize, // Index of the current stroke
    last_time: Option<f64>, // For tracking delta time 
}

impl KanjiAnimator {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            total_points: 0,
            progress: 0,
            is_playing: false,
            scale: 2.0,
            time_per_stroke: 1.0, 
            stroke_progress: 0.0,
            current_stroke_index: 0,
            last_time: None,
        }
    }

    // Load SVG file
    pub fn load_svg(&mut self, path: &str) -> anyhow::Result<()> {
        let raw_text = fs::read_to_string(path)?;

        let text = raw_text.replace("kvg:", "kvg_");

        let opt = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };

        let doc = Document::parse_with_options(&text, opt)
            .map_err(|e| anyhow::anyhow!("XML Error: {}", e))?;

        self.strokes.clear();
        self.total_points = 0;
        self.progress = 0;
        self.stroke_progress = 0.0;
        self.current_stroke_index = 0;
        self.last_time = None;
        self.is_playing = true;

        for node in doc.descendants() {
            if node.has_tag_name("path") {
                if let Some(d) = node.attribute("d") {
                    let points = parse_path_to_points(d, self.scale);
                    if !points.is_empty() {
                        self.total_points += points.len();
                        self.strokes.push(points);
                    }
                }
            }
        }

        Ok(())
    }

    // Function to draw the animation
    pub fn ui(&mut self, ui: &mut egui::Ui, available_rect: egui::Rect) {
        let painter = ui.painter_at(available_rect);
        let offset = available_rect.min;

        if self.strokes.is_empty() {
            return;
        }

        if !self.is_playing {
            self.draw_full_kanji(&painter, offset);
            return;
        }

        // Delta time
        let now = ui.input(|i| i.time);
        let delta = if let Some(prev) = self.last_time {
            (now - prev) as f32
        } else {
            0.016
        };
        self.last_time = Some(now);

        ui.ctx().request_repaint();

        self.stroke_progress += delta / self.time_per_stroke;

        while self.stroke_progress >= 1.0 && self.current_stroke_index < self.strokes.len() {
            self.stroke_progress -= 1.0;
            self.current_stroke_index += 1;
        }

        if self.current_stroke_index >= self.strokes.len() {
            self.is_playing = false;
            self.stroke_progress = 1.0;
        }

        let mut shapes = Vec::new();

        // Draw completed strokes fully + current partially
        for (i, stroke) in self.strokes.iter().enumerate() {
            let points_to_take = if i < self.current_stroke_index {
                stroke.len()      
            } else if i == self.current_stroke_index {
                ((stroke.len() as f32 * self.stroke_progress) as usize).max(2)
            } else {
                continue;               
            };

            let transformed: Vec<egui::Pos2> = stroke.iter()
                .take(points_to_take)
                .map(|p| egui::Pos2::new(p.x + offset.x, p.y + offset.y))
                .collect();

            if transformed.len() > 1 {
                shapes.push(egui::Shape::Path(egui::epaint::PathShape {
                    points: transformed,
                    closed: false,
                    fill: egui::Color32::TRANSPARENT,
                    stroke: egui::Stroke::new(3.5 * self.scale, egui::Color32::WHITE).into(),
                }));
            }
        }

        if !shapes.is_empty() {
            painter.extend(shapes);
        }
    }

    // Function to draw the full kanji
    fn draw_full_kanji(&self, painter: &egui::Painter, offset: egui::Pos2) {
        let mut shapes= Vec::new();

        for stroke in &self.strokes {
            let pts: Vec<egui::Pos2> = stroke.iter()
                .map(|p| egui::Pos2::new(p.x + offset.x, p.y + offset.y))
                .collect();

            if pts.len() > 1 {
                shapes.push(egui::Shape::Path(egui::epaint::PathShape {
                    points: pts,
                    closed: false,
                    fill: egui::Color32::TRANSPARENT,
                    stroke: egui::Stroke::new(3.5 * self.scale, egui::Color32::WHITE).into(),
                }));
            }
        }

        if !shapes.is_empty() {
            painter.extend(shapes);
        }
    }

    // Replay animation
    pub fn replay(&mut self) {
        self.progress = 0;
        self.stroke_progress = 0.0;
        self.current_stroke_index = 0;
        self.last_time = None;       
        self.is_playing = true;
    }

}


// Helper function to parse SVG path to points
fn parse_path_to_points(d: &str, scale: f32) -> Vec<egui::Pos2> {
    let mut path = BezPath::new();
    let mut current = Point::new(0.0, 0.0);

    // Use svgtypes to parse the path
    for segment in PathParser::from(d) {
        match segment {
            Ok(PathSegment::MoveTo { abs, x, y }) => {
                let mut px = x;
                let mut py = y;
                if !abs {
                    px += current.x;
                    py += current.y;
                }
                path.move_to(Point::new(px, py));
                current = Point::new(px, py);
            }
            Ok(PathSegment::LineTo { abs, x, y }) => {
                let mut px = x;
                let mut py = y;
                if !abs {
                    px += current.x;
                    py += current.y;
                }
                path.line_to(Point::new(px, py));
                current = Point::new(px, py);
            }
            Ok(PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y }) => {
                let mut px1 = x1;
                let mut py1 = y1;
                let mut px2 = x2;
                let mut py2 = y2;
                let mut px = x;
                let mut py = y;
                if !abs {
                    px1 += current.x;
                    py1 += current.y;
                    px2 += current.x;
                    py2 += current.y;
                    px += current.x;
                    py += current.y;
                }
                path.curve_to(Point::new(px1, py1), Point::new(px2, py2), Point::new(px, py));
                current = Point::new(px, py);
            }
            _ => {}
        }
    }

    let mut points = Vec::new();
    let tolerance = 0.01;
    // Use kurbo to convert the path to points
    flatten(path.iter(), tolerance, |el| {
        match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                points.push(egui::Pos2::new(p.x as f32 * scale, p.y as f32 * scale));
            }

            _ => {}
        } 
    });

    points
}