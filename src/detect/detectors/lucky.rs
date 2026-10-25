use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

const PATTERNS: &[&str] = &["777", "1234567"];

/// 縁起の良い並び(`777` / `1234567`)を部分文字列として検出する。スコアは長さ `k` に対する `4k` bits。
pub struct Lucky;

impl Detector for Lucky {
    fn id(&self) -> &'static str {
        "lucky"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        for pattern in PATTERNS {
            let mut from = 0;
            while let Some(pos) = hash[from..].find(pattern) {
                let start = from + pos;
                let end = start + pattern.len();
                hits.push(Hit {
                    detector_id: self.id(),
                    range: start..end,
                    score: bits_for_chars(pattern.len()),
                });
                from = start + 1;
            }
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_777() {
        let hits = Lucky.detect("a777b");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 1..4);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn detects_overlapping_777() {
        let hits = Lucky.detect("7777");
        let ranges: Vec<_> = hits.iter().map(|h| h.range.clone()).collect();
        assert_eq!(ranges, [0..3, 1..4]);
    }

    #[test]
    fn detects_1234567() {
        let hits = Lucky.detect("x1234567y");
        assert_eq!(hits[0].range, 1..8);
        assert_eq!(hits[0].score, 28.0);
    }
}
