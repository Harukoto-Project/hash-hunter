use std::ops::Range;

/// 検出器が見つけた 1 件のヒット。
#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub detector_id: &'static str,
    pub range: Range<usize>,
    pub score: f64,
}

/// 総合スコア付きのハッシュ。
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredHash {
    pub hash: String,
    pub hits: Vec<Hit>,
    pub score: f64,
}

/// レア判定を行う検出器。
pub trait Detector {
    /// 検出器 ID(kebab-case)を返す。
    fn id(&self) -> &'static str;

    /// ハッシュ文字列からヒットを検出する。
    fn detect(&self, hash: &str) -> Vec<Hit>;
}
