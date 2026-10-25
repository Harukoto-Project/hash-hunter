use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};

/// 最短周期が 2 文字以上の極大な繰り返しを検出する。周期 `p`・繰り返し `n` のスコアは `4p(n-1)` bits。
pub struct RepeatPattern;

impl Detector for RepeatPattern {
    fn id(&self) -> &'static str {
        "repeat-pattern"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let bytes = hash.as_bytes();
        let mut hits = Vec::new();
        let mut start = 0;
        while start < bytes.len() {
            if let Some((period, count)) = find_repeat(bytes, start) {
                let len = period * count;
                hits.push(Hit {
                    detector_id: self.id(),
                    range: start..start + len,
                    score: bits_for_chars(len - period),
                });
                start += len;
            } else {
                start += 1;
            }
        }
        hits
    }
}

fn find_repeat(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    let avail = bytes.len() - start;
    for period in 2..=avail / 2 {
        let unit = &bytes[start..start + period];
        if !is_primitive(unit) {
            continue;
        }
        if unit != &bytes[start + period..start + 2 * period] {
            continue;
        }
        let mut count = 2;
        while start + period * (count + 1) <= bytes.len()
            && unit == &bytes[start + period * count..start + period * (count + 1)]
        {
            count += 1;
        }
        return Some((period, count));
    }
    None
}

fn is_primitive(block: &[u8]) -> bool {
    let n = block.len();
    !(1..n).any(|d| n % d == 0 && block.chunks(d).all(|chunk| chunk == &block[..d]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_shortest_period_once() {
        let hits = RepeatPattern.detect("abababab");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 0..8);
        assert_eq!(hits[0].score, 24.0);
    }

    #[test]
    fn detects_period_three() {
        let hits = RepeatPattern.detect("abcabcabc");
        assert_eq!(hits[0].range, 0..9);
        assert_eq!(hits[0].score, 24.0);
    }

    #[test]
    fn ignores_single_char_period() {
        assert!(RepeatPattern.detect("aaaaaaaa").is_empty());
    }

    #[test]
    fn detects_adjacent_patterns() {
        let ranges: Vec<_> = RepeatPattern
            .detect("abababcdcdcd")
            .iter()
            .map(|h| h.range.clone())
            .collect();
        assert_eq!(ranges, [0..6, 6..12]);
    }
}
