use std::collections::BTreeSet;

use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

const MIN_LEN: usize = 6;

/// 長さ 6 以上の極大な回文部分文字列を検出する。長さ `k` のスコアは `4 * floor(k/2)` bits。
pub struct Palindrome;

impl Detector for Palindrome {
    fn id(&self) -> &'static str {
        "palindrome"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let bytes = hash.as_bytes();
        let mut ranges = BTreeSet::new();
        for center in 0..bytes.len() {
            if let Some(range) = expand(bytes, center, center) {
                ranges.insert((range.start, range.end));
            }
            if center + 1 < bytes.len() {
                if let Some(range) = expand(bytes, center, center + 1) {
                    ranges.insert((range.start, range.end));
                }
            }
        }
        ranges
            .into_iter()
            .filter(|(start, end)| end - start >= MIN_LEN)
            .map(|(start, end)| Hit {
                detector_id: self.id(),
                range: start..end,
                score: bits_for_chars((end - start) / 2),
            })
            .collect()
    }
}

fn expand(bytes: &[u8], mut lo: usize, mut hi: usize) -> Option<std::ops::Range<usize>> {
    if bytes[lo] != bytes[hi] {
        return None;
    }
    while lo > 0 && hi + 1 < bytes.len() && bytes[lo - 1] == bytes[hi + 1] {
        lo -= 1;
        hi += 1;
    }
    Some(lo..hi + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_even_palindrome() {
        let hits = Palindrome.detect("xabccbay");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 1..7);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn detects_odd_palindrome() {
        let hits = Palindrome.detect("abcdcba");
        assert_eq!(hits[0].range, 0..7);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn ignores_short_palindrome() {
        assert!(Palindrome.detect("abba12").is_empty());
    }

    #[test]
    fn reports_maximal_range_once() {
        let hits = Palindrome.detect("aaaaaa");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 0..6);
    }
}
