use eframe::egui;
use kurbo::{BezPath, Point, PathEl};
use roxmltree::{Document, ParsingOptions};
use std::fs;
use svgtypes::{PathParser, PathSegment};

// Stroke Point Data
#[derive(Clone, Debug)]
struct StrokePoint {
    pos: Point,
    dist: f32,
}

// Stroke Data
#[derive(Clone)]
struct Stroke {
    points: Vec<StrokePoint>,
    total_length: f32,
}

// Kanji Animator
pub struct KanjiAnimator {
    strokes: Vec<Stroke>, // Stroke Data
    is_playing: bool, // Is Playing
    stroke_progress: f32, // Stroke Progress
    current_stroke_index: usize, // Current Stroke Index
    last_time: Option<f64>, // Last Time 
}

impl KanjiAnimator {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            is_playing: false,
            stroke_progress: 0.0,
            current_stroke_index: 0,
            last_time: None,
        }
    }

    // Load SVG
    pub fn load_svg(&mut self, path: &str) -> anyhow::Result<()> {
        self.strokes.clear();
        self.is_playing = false;
        self.stroke_progress = 0.0;
        self.current_stroke_index = 0;
        self.last_time = None;

        let raw_text = fs::read_to_string(path)?;
        let text = raw_text.replace("kvg:", "kvg_");
        
        let opt = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };
        let doc = Document::parse_with_options(&text, opt)
            .map_err(|e| anyhow::anyhow!("XML Error: {}", e))?;

        self.is_playing = true;

        for node in doc.descendants() {
            if node.has_tag_name("path") {
                if let Some(d) = node.attribute("d") {
                    let mut bez_path = BezPath::new();
                    let mut current = Point::ZERO;

                    // Parse SVG
                    for seg in PathParser::from(d) {
                        match seg? {
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
                                let c1 = if abs { Point::new(x1, y1) } else { current + (x1, y1)};
                                let c2 = if abs { Point::new(x2, y2) } else { current + (x2, y2) };
                                let p = if abs { Point::new(x, y) } else { current + (x, y) };
                                bez_path.curve_to(c1, c2, p);
                                current = p;
                            }
                            _ => {}
                        }
                    }

                    let mut points = Vec::new();
                    let mut total_dist = 0.0;
                    let mut is_first = true;
                    let mut last_pos = Point::ZERO; // Save the last position

                    kurbo::flatten(bez_path.iter(), 0.2, |el| {
                        match el {
                            // Start of a stroke
                            PathEl::MoveTo(p) => {
                                last_pos = p;
                                if is_first {
                                    points.push(StrokePoint {
                                        pos: p,
                                        dist: 0.0,
                                    });
                                    is_first = false;
                                }
                            }
                            // Continue of a stroke
                            PathEl::LineTo(p) => {
                                let dist = last_pos.distance(p);
                                total_dist += dist;
                                
                                points.push(StrokePoint {
                                    pos: p,
                                    dist: total_dist as f32,
                                });
                                last_pos = p; // Update last position
                            }
                            _ => {}
                        }
                    });

                    if !points.is_empty() {
                        self.strokes.push(Stroke {
                            points,
                            total_length: total_dist as f32,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    // Replay animation
    pub fn replay(&mut self) {
        if !self.strokes.is_empty() {
            self.current_stroke_index = 0;
            self.stroke_progress = 0.0;
            self.last_time = None;
            self.is_playing = true;
        }
    }

    // Clear animation
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.is_playing = false;
        self.stroke_progress = 0.0;
        self.current_stroke_index = 0;
        self.last_time = None;
    }

    // Display animation
    pub fn ui(&mut self, ui: &mut egui::Ui, rect: egui::Rect, char_to_show: &str, speed: f32) {
        let painter = ui.painter_at(rect);

        if self.strokes.is_empty() {
            let font_size = rect.height() * 0.8;

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                char_to_show,
                egui::FontId::proportional(font_size),
                egui::Color32::WHITE,
            );

            return;
        }

        // Scale
        let desired_size = 109.0;
        let scale = (rect.width().min(rect.height())) / desired_size;
        
        let offset_x = rect.min.x + (rect.width() - desired_size * scale) / 2.0;
        let offset_y = rect.min.y + (rect.height() - desired_size * scale) / 2.0;
        let offset = egui::vec2(offset_x, offset_y);

        let transform = |p: Point| -> egui::Pos2 {
            egui::pos2(
                (p.x as f32 * scale) + offset.x,
                (p.y as f32 * scale) + offset.y,
            )
        };

        // Logic of time
        if self.is_playing {
            let now = ui.input(|i| i.time);
            let delta = if let Some(last) = self.last_time {
                (now - last) as f32
            } else {
                0.0
            };
            self.last_time = Some(now);

            self.stroke_progress += delta / speed;

            if self.stroke_progress >= 1.0 {
                self.stroke_progress = 0.0;
                self.current_stroke_index += 1;
                
                if self.current_stroke_index >= self.strokes.len() {
                    self.is_playing = false;
                    self.current_stroke_index = self.strokes.len(); 
                }
            }
            ui.ctx().request_repaint();
        }

        let stroke_width = 4.0 * scale;

        // Drawing
        for (i, stroke) in self.strokes.iter().enumerate() {
            let color = egui::Color32::WHITE;

            if i < self.current_stroke_index {
                // Draw full stroke
                let screen_points: Vec<egui::Pos2> = stroke.points.iter()
                    .map(|sp| transform(sp.pos))
                    .collect();
                
                if screen_points.len() > 1 {
                    painter.add(egui::Shape::Path(egui::epaint::PathShape {
                        points: screen_points,
                        closed: false,
                        fill: egui::Color32::TRANSPARENT,
                        stroke: egui::Stroke::new(stroke_width, color).into(),
                    }));
                }
            } else if i == self.current_stroke_index && self.is_playing {
                // Draw active stroke with interpolation
                let target_len = stroke.total_length * self.stroke_progress;
                let mut screen_points = Vec::new();

                for (idx, sp) in stroke.points.iter().enumerate() {
                    if sp.dist <= target_len {
                        screen_points.push(transform(sp.pos));
                    } else {
                        if idx > 0 {
                            let prev = &stroke.points[idx - 1];
                            let segment_len = sp.dist - prev.dist;
                            
                            if segment_len > 0.0001 {
                                let dist_needed = target_len - prev.dist;
                                let t = (dist_needed / segment_len) as f64;
                                
                                let x = prev.pos.x + (sp.pos.x - prev.pos.x) * t;
                                let y = prev.pos.y + (sp.pos.y - prev.pos.y) * t;
                                
                                screen_points.push(transform(Point::new(x, y)));
                            }
                        }
                        break;
                    }
                }

                if screen_points.len() > 1 {
                    painter.add(egui::Shape::Path(egui::epaint::PathShape {
                        points: screen_points,
                        closed: false,
                        fill: egui::Color32::TRANSPARENT,
                        stroke: egui::Stroke::new(stroke_width, color).into(),
                    }));
                }
            }
        }
    }
}