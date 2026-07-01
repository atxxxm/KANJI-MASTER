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

    /// True once the SVG templates have been parsed into the cache.
    pub fn is_loaded(&self) -> bool {
        !self.cache.is_empty()
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