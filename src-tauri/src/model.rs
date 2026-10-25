use serde::{Deserialize, Serialize};

/// スキャン条件。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanInput {
    pub repo: String,
    pub top: usize,
    pub min_score: f64,
    pub branch: Option<String>,
    pub only: Vec<String>,
    pub exclude: Vec<String>,
}

/// 1 件のヒット。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitDto {
    pub detector_id: String,
    pub start: usize,
    pub end: usize,
    pub score: f64,
}

/// ランキング 1 件。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedHashDto {
    pub hash: String,
    pub score: f64,
    pub hits: Vec<HitDto>,
}

/// 直近スキャンの条件と結果。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSnapshot {
    pub input: ScanInput,
    pub ranking: Vec<RankedHashDto>,
}

/// フロントへ返すコマンドエラー。
#[derive(Debug, Serialize)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl From<hash_hunter::error::AppError> for CommandError {
    fn from(err: hash_hunter::error::AppError) -> Self {
        Self {
            code: err.code().to_string(),
            message: err.to_string(),
        }
    }
}
