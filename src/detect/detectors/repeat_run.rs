use crate::detect::bits::bits_for_chars;
use crate::detect::model::{Detector, Hit};
use crate::detect::MIN_RUN_LEN;

/// 同じ文字が 4 文字以上連続している部分を検出する。
/// 長さ `k` の連続が特定の位置から始まる確率 `16^-(k-1)` に対する `4(k-1)` bits をスコアとする。
pub struct RepeatRun;

impl Detector for RepeatRun {
    fn id(&self) -> &'static str {
        "repeat-run"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        let bytes = hash.as_bytes();
        let mut hits = Vec::new();
        let mut start = 0;
        while start < bytes.len() {
            let len = bytes[start..]
                .iter()
                .take_while(|&&b| b == bytes[start])
                .count();
            if len >= MIN_RUN_LEN {
                hits.push(Hit {
                    detector_id: self.id(),
                    range: start..start + len,
                    score: bits_for_chars(len - 1),
                });
            }
            start += len;
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_run_of_same_char() {
        let hits = RepeatRun.detect("a7777b");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 1..5);
        assert_eq!(hits[0].score, 12.0);
    }

    #[test]
    fn detects_multiple_runs() {
        let hits = RepeatRun.detect("aaaa1111");
        let ranges: Vec<_> = hits.iter().map(|h| h.range.clone()).collect();
        assert_eq!(ranges, [0..4, 4..8]);
    }

    #[test]
    fn ignores_short_run() {
        assert!(RepeatRun.detect("aaa111").is_empty());
    }

    #[test]
    fn no_hit_without_repetition() {
        assert!(RepeatRun.detect("abcdef").is_empty());
    }
}
