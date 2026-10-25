//! アプリ共通のエラー型。

use std::fmt;

/// アプリ全体で扱うエラー。`code` はエラーコード(snake_case)に対応する。
#[derive(Debug)]
pub enum AppError {
    GitNotFound,
    CloneFailed(String),
    EmptyRepository,
    UnknownDetector(String),
}

impl AppError {
    /// エラーコードを返す。
    pub fn code(&self) -> &'static str {
        match self {
            AppError::GitNotFound => "git_not_found",
            AppError::CloneFailed(_) => "clone_failed",
            AppError::EmptyRepository => "empty_repository",
            AppError::UnknownDetector(_) => "unknown_detector",
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::GitNotFound => write!(f, "git command was not found"),
            AppError::CloneFailed(detail) => write!(f, "failed to clone repository: {detail}"),
            AppError::EmptyRepository => write!(f, "repository has no commits"),
            AppError::UnknownDetector(id) => write!(f, "unknown detector: {id}"),
        }
    }
}

impl std::error::Error for AppError {}
