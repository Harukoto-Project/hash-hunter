use std::ops::Range;

use crate::detect::MIN_RUN_LEN;

/// 16 進 1 桁を 0..=15 に変換する。
pub fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// 各桁が直前の桁 + `step` になっている極大な連続を返す。長さ 4 未満は含めない。
pub fn hex_step_runs(hash: &str, step: i8) -> Vec<Range<usize>> {
    let bytes = hash.as_bytes();
    let mut runs = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let Some(mut prev) = hex_digit(bytes[i]) else {
            i += 1;
            continue;
        };
        let mut len = 1;
        while i + len < bytes.len() {
            let Some(digit) = hex_digit(bytes[i + len]) else {
                break;
            };
            let expected = i16::from(prev) + i16::from(step);
            if !(0..=15).contains(&expected) || i16::from(digit) != expected {
                break;
            }
            prev = digit;
            len += 1;
        }
        if len >= MIN_RUN_LEN {
            runs.push(i..i + len);
        }
        i += len;
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_digit_accepts_both_cases() {
        assert_eq!(hex_digit(b'0'), Some(0));
        assert_eq!(hex_digit(b'a'), Some(10));
        assert_eq!(hex_digit(b'F'), Some(15));
        assert_eq!(hex_digit(b'g'), None);
    }

    #[test]
    fn step_runs_are_maximal_and_unwrapped() {
        assert_eq!(hex_step_runs("12345", 1), vec![0..5]);
        assert_eq!(hex_step_runs("cdef0123", 1), vec![0..4, 4..8]);
        assert_eq!(hex_step_runs("ef01", 1), Vec::<Range<usize>>::new());
        assert_eq!(hex_step_runs("fedcba", -1), vec![0..6]);
    }
}
