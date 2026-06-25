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
    pub fn search(&self, user_strokes: &[Vec<(f32, f32)>], limit: usize) -> Vec<(i32, f32)> {
        if user_strokes.is_empty() {
            return Vec::new();
        }

        let user_normalized = normalize_kanji(user_strokes.to_vec());
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

// Resample a stroke to exactly `n` evenly-spaced points by arc length.
// Uses the "insert pivot" technique: after placing an interpolated point q,
// q is inserted into the working list so the next pass continues from q
// within the same segment instead of jumping to the next vertex.
fn resample(stroke: &[(f32, f32)], n: usize) -> Vec<(f32, f32)> {
    if stroke.len() < 2 {
        let p = stroke.first().copied().unwrap_or((0.0, 0.0));
        return vec![p; n];
    }

    let total: f32 = stroke.windows(2).map(|w| dist(w[0], w[1])).sum();
    if total == 0.0 {
        return vec![stroke[0]; n];
    }

    let step = total / (n - 1) as f32;
    let mut result = vec![stroke[0]];
    let mut accumulated = 0.0_f32;
    let mut pts: Vec<(f32, f32)> = stroke.to_vec();
    let mut i = 1;

    while i < pts.len() && result.len() < n - 1 {
        let seg_len = dist(pts[i - 1], pts[i]);
        if accumulated + seg_len >= step {
            let t = (step - accumulated) / seg_len;
            let q = (
                pts[i - 1].0 + t * (pts[i].0 - pts[i - 1].0),
                pts[i - 1].1 + t * (pts[i].1 - pts[i - 1].1),
            );
            result.push(q);
            pts.insert(i, q);
            accumulated = 0.0;
        } else {
            accumulated += seg_len;
        }
        i += 1;
    }

    while result.len() < n {
        result.push(*pts.last().unwrap());
    }
    result
}

fn normalize_kanji(strokes: Vec<Vec<(f32, f32)>>) -> Vec<Vec<(f32, f32)>> {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for stroke in &strokes {
        for &(x, y) in stroke {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }
    }

    let scale = (max_x - min_x).max(max_y - min_y);
    if scale == 0.0 {
        return strokes;
    }

    strokes
        .into_iter()
        .filter(|s| s.len() >= 2)
        .map(|stroke| {
            resample(&stroke, POINTS_PER_STROKE)
                .into_iter()
                .map(|(x, y)| ((x - min_x) / scale, (y - min_y) / scale))
                .collect()
        })
        .collect()
}

fn dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

// Algorithm for similarity calculation
fn calculate_similarity(user: &[Vec<(f32, f32)>], template: &[Vec<(f32, f32)>]) -> f32 {
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