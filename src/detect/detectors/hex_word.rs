use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

const WORDS: &[&str] = &["dead", "beef", "cafe", "babe", "face", "c0de", "f00d"];

/// 16 進で読める単語を部分文字列として検出する。スコアは単語長 `k` に対する `4k` bits。
pub struct HexWord;

impl Detector for HexWord {
    fn id(&self) -> &'static str {
        "hex-word"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let lower = hash.to_ascii_lowercase();
        let mut hits = Vec::new();
        for word in WORDS {
            let mut from = 0;
            while let Some(pos) = lower[from..].find(word) {
                let start = from + pos;
                let end = start + word.len();
                hits.push(Hit {
                    detector_id: self.id(),
                    range: start..end,
                    score: bits_for_chars(word.len()),
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
    fn detects_listed_words() {
        let hits = HexWord.detect("00deadbeef");
        let ranges: Vec<_> = hits.iter().map(|h| h.range.clone()).collect();
        assert_eq!(ranges, [2..6, 6..10]);
        assert_eq!(hits[0].score, 16.0);
        assert_eq!(hits[1].score, 16.0);
    }

    #[test]
    fn is_case_insensitive() {
        let hits = HexWord.detect("CAFE");
        assert_eq!(hits[0].range, 0..4);
    }

    #[test]
    fn ignores_unknown_words() {
        assert!(HexWord.detect("00fade00").is_empty());
    }
}
