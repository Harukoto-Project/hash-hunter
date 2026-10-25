use crate::detect::bits::bits_for_chars;
use crate::detect::hex::hex_step_runs;
use crate::detect::model::{Detector, Hit};

/// 16 進で連続した降順(4 文字以上)を検出する。長さ `k` のスコアは `4(k-1)` bits。
pub struct Descending;

impl Detector for Descending {
    fn id(&self) -> &'static str {
        "descending"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        hex_step_runs(hash, -1)
            .into_iter()
            .map(|range| Hit {
                detector_id: self.id(),
                score: bits_for_chars(range.end - range.start - 1),
                range,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_hex_descent() {
        let hits = Descending.detect("a54321f");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 1..6);
        assert_eq!(hits[0].score, 16.0);
    }

    #[test]
    fn treats_ba_as_descent() {
        let hits = Descending.detect("cba9");
        assert_eq!(hits[0].range, 0..4);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn ignores_short_descent() {
        assert!(Descending.detect("fed1").is_empty());
    }
}
