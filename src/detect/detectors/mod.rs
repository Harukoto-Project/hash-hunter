//! 検出器の実装。1 検出器 = 1 ファイル。

mod ascending;
mod descending;
mod digits_only;
mod hex_word;
mod leading_zeros;
mod letters_only;
mod lucky;
mod palindrome;
mod repeat_pattern;
mod repeat_run;
mod trailing_zeros;

pub use ascending::Ascending;
pub use descending::Descending;
pub use digits_only::DigitsOnly;
pub use hex_word::HexWord;
pub use leading_zeros::LeadingZeros;
pub use letters_only::LettersOnly;
pub use lucky::Lucky;
pub use palindrome::Palindrome;
pub use repeat_pattern::RepeatPattern;
pub use repeat_run::RepeatRun;
pub use trailing_zeros::TrailingZeros;
