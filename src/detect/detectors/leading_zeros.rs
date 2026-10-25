use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

/// 先頭が `0` で連続している部分を検出する。長さ `k` のスコアは `16^-k` に対する `4k` bits。
pub struct LeadingZeros;

impl Detector for LeadingZeros {
    fn id(&self) -> &'static str {
        "leading-zeros"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let len = hash.bytes().take_while(|&b| b == b'0').count();
        if len == 0 {
            return Vec::new();
        }
        vec![Hit {
            detector_id: self.id(),
            range: 0..len,
            score: bits_for_chars(len),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_leading_zero_run() {
        let hits = LeadingZeros.detect("0000a3f1");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 0..4);
        assert_eq!(hits[0].score, 16.0);
    }

    #[test]
    fn ignores_zeros_not_at_start() {
        assert!(LeadingZeros.detect("a0000b").is_empty());
    }

    #[test]
    fn single_leading_zero_is_a_hit() {
        let hits = LeadingZeros.detect("0abc");
        assert_eq!(hits[0].range, 0..1);
        assert_eq!(hits[0].score, 4.0);
    }

    #[test]
    fn all_zeros_covers_whole_hash() {
        let hits = LeadingZeros.detect("0000");
        assert_eq!(hits[0].range, 0..4);
    }
}
