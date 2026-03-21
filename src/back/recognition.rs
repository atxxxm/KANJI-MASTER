use crate::back::svg_cache::Stroke;
use std::collections::HashMap;

// Number of points to which each stroke is reduced
const POINTS_PER_STROKE: usize = 32;

// Simplified kanji caching struct
#[derive(Clone, Debug)]
pub struct SimplifiedKanji {
    pub id: i32,
    // Normalized strokes (0.0..1.0)
    pub strokes: Vec<Vec<(f32, f32)>>,
}

// Recognition system
pub struct RecognitionSystem {
    cache: Vec<SimplifiedKanji>,
}

impl RecognitionSystem {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    pub fn load_cache(&mut self, svg_data: &HashMap<i32, Vec<Stroke>>) {
        self.cache.clear();
        
        for (id, strokes) in svg_data {
            let raw_strokes: Vec<Vec<(f32, f32)>> = strokes.iter()
                .map(|stroke| {
                    stroke.points.iter()
                        .map(|sp| (sp.pos.x as f32, sp.pos.y as f32))
                        .collect()
                })
                .collect();

            let normalized = normalize_kanji(raw_strokes);
            self.cache.push(SimplifiedKanji {
                id: *id,
                strokes: normalized,
            });
        }
    }

    // Search for similar kanji
    pub fn search(&self, user_strokes: &Vec<Vec<(f32, f32)>>, limit: usize) -> Vec<(i32, f32)> {
        if user_strokes.is_empty() {
            return Vec::new();
        }

        let user_normalized = normalize_kanji(user_strokes.clone());
        let user_stroke_count = user_normalized.len();
        
        let mut scores: Vec<(i32, f32)> = self.cache.iter()
            .filter(|k| {
                // Optimization: skip if stroke count is too different
                let diff = (k.strokes.len() as i32 - user_stroke_count as i32).abs();
                diff <= 2
            })
            .map(|k| {
                let score = calculate_similarity(&user_normalized, &k.strokes);
                (k.id, score)
            })
            .collect();

        // Sort: lower score is better match
        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        scores.into_iter().take(limit).collect()
    }
}

// Support functions

// Resampling and normalization
fn normalize_kanji(strokes: Vec<Vec<(f32, f32)>>) -> Vec<Vec<(f32, f32)>> {
    // Find bounding box
    let mut min_x = f32::MAX; let mut max_x = f32::MIN;
    let mut min_y = f32::MAX; let mut max_y = f32::MIN;

    for stroke in &strokes {
        for p in stroke {
            if p.0 < min_x { min_x = p.0; }
            if p.0 > max_x { max_x = p.0; }
            if p.1 < min_y { min_y = p.1; }
            if p.1 > max_y { max_y = p.1; }
        }
    }

    let width = max_x - min_x;
    let height = max_y - min_y;
    let scale = if width > height { width } else { height }; // Save aspect ratio, fitting into square
    
    if scale == 0.0 { return strokes; }

    // Resampling each stroke
    let mut resampled_strokes = Vec::new();

    for stroke in strokes {
        if stroke.len() < 2 { continue; }
        
        let mut new_stroke = Vec::new();
        
        // Calculate total length of stroke
        let mut total_len = 0.0;
        for i in 0..stroke.len()-1 {
            total_len += dist(stroke[i], stroke[i+1]);
        }
        
        let step = total_len / (POINTS_PER_STROKE as f32 - 1.0);
        
        // Add points
        let mut current_dist = 0.0;
        new_stroke.push(stroke[0]); // First point
        
        let mut src_idx = 0;
        // Simplified linear interpolation by length
        while new_stroke.len() < POINTS_PER_STROKE {
            if src_idx >= stroke.len() - 1 {
                 new_stroke.push(*stroke.last().unwrap());
                 continue;
            }
            
            let p1 = stroke[src_idx];
            let p2 = stroke[src_idx+1];
            let d = dist(p1, p2);
            
            if current_dist + d >= step {
                let remaining = step - current_dist;
                let t = remaining / d;
                let new_x = p1.0 + (p2.0 - p1.0) * t;
                let new_y = p1.1 + (p2.1 - p1.1) * t;
                let new_p = (new_x, new_y);
                new_stroke.push(new_p);
                
                 current_dist = 0.0;
            } else {
                current_dist += d;
            }
            src_idx += 1;
        }
        
        // Add the last point if needed
        while new_stroke.len() < POINTS_PER_STROKE {
            new_stroke.push(*stroke.last().unwrap());
        }

        // Normalize coordinates of each point in the stroke
        let mut normalized_stroke = Vec::new();
        for p in new_stroke {
             let nx = (p.0 - min_x) / scale;
             let ny = (p.1 - min_y) / scale;
             normalized_stroke.push((nx, ny));
        }
        resampled_strokes.push(normalized_stroke);
    }
    
    resampled_strokes
}

fn dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

// Algorithm for similarity calculation
fn calculate_similarity(user: &Vec<Vec<(f32, f32)>>, template: &Vec<Vec<(f32, f32)>>) -> f32 {
    let mut total_score = 0.0;
    
    // Compare the N-th stroke of the user with the N-th stroke of the template
    // If the number of strokes is different, "extra" strokes give a large penalty
    let count = std::cmp::max(user.len(), template.len());
    
    for i in 0..count {
        if i < user.len() && i < template.len() {
            // Compare points within each stroke
            let u_stroke = &user[i];
            let t_stroke = &template[i];
            let points = std::cmp::min(u_stroke.len(), t_stroke.len());
            
            let mut stroke_score = 0.0;
            for j in 0..points {
                stroke_score += dist(u_stroke[j], t_stroke[j]);
            }
            total_score += stroke_score;
        } else {
            // Penalty for missing/extra stroke 
            total_score += 10.0; 
        }
    }
    
    total_score
}