//! 結果の表示。

mod ranking;

pub use ranking::{print_ranking, rank};

/// 検出器 ID を 1 行ずつ標準出力する。
pub fn print_detector_list(ids: &[&str]) {
    for id in ids {
        println!("{id}");
    }
}
