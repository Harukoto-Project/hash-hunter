use crate::detect::bits::bits_for_restricted_alphabet;
use crate::detect::model::{Detector, Hit};

/// ハッシュ全体が数字(`0-9`)だけのときヒットする。
pub struct DigitsOnly;

impl Detector for DigitsOnly {
    fn id(&self) -> &'static str {
        "digits-only"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        if hash.is_empty() || !hash.bytes().all(|b| b.is_ascii_digit()) {
            return Vec::new();
        }
        vec![Hit {
            detector_id: self.id(),
            range: 0..hash.len(),
            score: bits_for_restricted_alphabet(hash.len(), 10),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_all_digits() {
        let hits = DigitsOnly.detect("1984327");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 0..7);
        assert_eq!(hits[0].score, bits_for_restricted_alphabet(7, 10));
    }

    #[test]
    fn rejects_letters() {
        assert!(DigitsOnly.detect("198a327").is_empty());
    }
}
