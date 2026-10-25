use crate::detect::model::{Detector, ScoredHash};

/// 全検出器に `hash` を通し、ヒットと総合スコア(全ヒットの単純合計)を返す。
pub fn score_hash(hash: &str, detectors: &[Box<dyn Detector>]) -> ScoredHash {
    let hits: Vec<_> = detectors.iter().flat_map(|d| d.detect(hash)).collect();
    let score = hits.iter().map(|hit| hit.score).sum();
    ScoredHash {
        hash: hash.to_string(),
        hits,
        score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::registry;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn sums_scores_of_all_hits() {
        let detectors = registry::select(&strings(&["leading-zeros", "repeat-run"]), &[]).unwrap();
        let scored = score_hash("01a7777", &detectors);
        assert_eq!(scored.hits.len(), 2);
        assert_eq!(scored.score, 4.0 + 12.0);
    }

    #[test]
    fn no_hits_gives_zero_score() {
        let detectors = registry::all();
        let scored = score_hash("1a2b3c", &detectors);
        assert!(scored.hits.is_empty());
        assert_eq!(scored.score, 0.0);
    }
}
