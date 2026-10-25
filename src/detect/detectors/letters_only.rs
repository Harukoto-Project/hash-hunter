use crate::detect::bits::bits_for_restricted_alphabet;
use crate::detect::hex::hex_digit;
use crate::detect::model::{Detector, Hit};

/// ハッシュ全体が英字(`a-f`)だけのときヒットする。
pub struct LettersOnly;

impl Detector for LettersOnly {
    fn id(&self) -> &'static str {
        "letters-only"
    }

    fn detect(&self, hash: &str) -> Vec<Hit> {
        if hash.is_empty() || !hash.bytes().all(|b| hex_digit(b).is_some_and(|d| d >= 10)) {
            return Vec::new();
        }
        vec![Hit {
            detector_id: self.id(),
            range: 0..hash.len(),
            score: bits_for_restricted_alphabet(hash.len(), 6),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_all_letters() {
        let hits = LettersOnly.detect("deadbeef");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].range, 0..8);
        assert_eq!(hits[0].score, bits_for_restricted_alphabet(8, 6));
    }

    #[test]
    fn rejects_digits() {
        assert!(LettersOnly.detect("dead0eef").is_empty());
    }
}
