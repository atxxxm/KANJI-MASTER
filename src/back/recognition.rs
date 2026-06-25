use crate::back::core::Kanji;
use crate::back::svg_cache::parse_svg_content;
use std::fs;
use std::sync::Arc;

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

    pub fn load_from_svgs(&mut self, kanji_db: &[Arc<Kanji>], svg_path: &str) {
        self.cache.clear();
        for k in kanji_db {
            let file_path = format!("{}/0{}.svg", svg_path, k.unicode.to_lowercase());
            let Ok(content) = fs::read_to_string(&file_path) else { continue };
            let Some(strokes) = parse_svg_content(&content) else { continue };
            let raw_strokes: Vec<Vec<(f32, f32)>> = strokes
                .iter()
                .map(|s| s.points.iter().map(|sp| (sp.pos.x as f32, sp.pos.y as f32)).collect())
                .collect();
            let normalized = normalize_kanji(raw_strokes);
            self.cache.push(SimplifiedKanji { id: k.id, strokes: normalized });
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

// DTW distance between two strokes using two-row optimization (O(n*m) time, O(m) space).
fn dtw_distance(s1: &[(f32, f32)], s2: &[(f32, f32)]) -> f32 {
    let n = s1.len();
    let m = s2.len();
    let mut prev = vec![f32::INFINITY; m + 1];
    let mut curr = vec![f32::INFINITY; m + 1];
    prev[0] = 0.0;

    for i in 1..=n {
        curr[0] = f32::INFINITY;
        for j in 1..=m {
            let cost = dist(s1[i - 1], s2[j - 1]);
            curr[j] = cost + prev[j].min(curr[j - 1]).min(prev[j - 1]);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

// Greedy best-match pairing: each user stroke is matched to the closest unmatched
// template stroke. Order-independent — handles strokes drawn out of canonical sequence.
fn calculate_similarity(user: &[Vec<(f32, f32)>], template: &[Vec<(f32, f32)>]) -> f32 {
    let mut used = vec![false; template.len()];
    let mut total = 0.0;

    for u_stroke in user {
        let mut best_dist = f32::INFINITY;
        let mut best_j = 0;
        for (j, t_stroke) in template.iter().enumerate() {
            if !used[j] {
                let d = dtw_distance(u_stroke, t_stroke);
                if d < best_dist {
                    best_dist = d;
                    best_j = j;
                }
            }
        }
        if best_dist < f32::INFINITY {
            used[best_j] = true;
            total += best_dist;
        }
    }

    // Penalty for each unmatched template stroke
    let unmatched = used.iter().filter(|&&u| !u).count();
    total += unmatched as f32 * 10.0;

    total
}