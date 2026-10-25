use crate::detect::bits::bits_for_chars;
use crate::detect::hex::hex_step_runs;
use crate::detect::model::{Detector, Hit};

/// 16 進で連続した昇順(4 文字以上)を検出する。長さ `k` のスコアは `4(k-1)` bits。
pub struct Ascending;

impl Detector for Ascending {
    fn id(&self) -> &'static str {
        "ascending"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        hex_step_runs(hash, 1)
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
    fn detects_hex_ascent() {
        let hits = Ascending.detect("x12345f");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 1..6);
        assert_eq!(hits[0].score, 16.0);
    }

    #[test]
    fn treats_9a_as_ascent() {
        let hits = Ascending.detect("89ab");
        assert_eq!(hits[0].range, 0..4);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn does_not_wrap_around() {
        let ranges: Vec<_> = Ascending
            .detect("cdef0123")
            .iter()
            .map(|h| h.range.clone())
            .collect();
        assert_eq!(ranges, [0..4, 4..8]);
    }

    #[test]
    fn ignores_short_ascent() {
        assert!(Ascending.detect("ef01").is_empty());
    }
}
