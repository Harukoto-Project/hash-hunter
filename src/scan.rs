//! リポジトリからコミットハッシュを集め、検出器で採点する。

use crate::detect::{self, ScoredHash};
use crate::error::AppError;
use crate::git;

/// リポジトリのコミットを検出し、総合スコア付きの一覧を返す。
pub fn collect(
    repo: &str,
    branch: Option<&str>,
    only: &[String],
    exclude: &[String],
) -> Result<Vec<ScoredHash>, AppError> {
    let detectors = detect::select(only, exclude)?;
    let workspace = git::Workspace::open(repo)?;
    let commits = git::list_commits(workspace.path(), branch)?;
    Ok(commits
        .iter()
        .map(|commit| detect::score_hash(&commit.hash, &detectors))
        .collect())
}
