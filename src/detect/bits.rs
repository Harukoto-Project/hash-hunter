const BITS_PER_CHAR: f64 = 4.0;

/// 16 進 `count` 文字が偶然一致する確率に対するスコア(bits)を返す。
pub fn bits_for_chars(count: usize) -> f64 {
    BITS_PER_CHAR * count as f64
}

/// 各桁が `alphabet_size` 通りに制限される長さ `len` のスコアを返す。
pub fn bits_for_restricted_alphabet(len: usize, alphabet_size: usize) -> f64 {
    (len as f64) * (BITS_PER_CHAR - (alphabet_size as f64).log2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_bits_per_hex_char() {
        assert_eq!(bits_for_chars(0), 0.0);
        assert_eq!(bits_for_chars(1), 4.0);
        assert_eq!(bits_for_chars(5), 20.0);
    }

    #[test]
    fn restricted_alphabet_is_len_times_log_ratio() {
        let expected = 6.0 * (4.0 - 10f64.log2());
        assert_eq!(bits_for_restricted_alphabet(6, 10), expected);
    }
}
