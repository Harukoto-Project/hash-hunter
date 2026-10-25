use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

/// 末尾が `0` で連続している部分を検出する。長さ `k` のスコアは `16^-k` に対する `4k` bits。
pub struct TrailingZeros;

impl Detector for TrailingZeros {
    fn id(&self) -> &'static str {
        "trailing-zeros"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let len = hash.bytes().rev().take_while(|&b| b == b'0').count();
        if len == 0 {
            return Vec::new();
        }
        vec![Hit {
            detector_id: self.id(),
            range: (hash.len() - len)..hash.len(),
            score: bits_for_chars(len),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_trailing_zero_run() {
        let hits = TrailingZeros.detect("b1000");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 2..5);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn ignores_zeros_not_at_end() {
        assert!(TrailingZeros.detect("0000a").is_empty());
    }
}
