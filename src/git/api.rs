use std::io::ErrorKind;
use std::path::Path;
use std::process::{Command, Output};

use crate::error::AppError;
use crate::git::model::Commit;

/// `git` コマンドが実行できることを確認する。
pub fn ensure_git_available() -> Result<(), AppError> {
    run_git(&["--version"]).map(|_| ())
}

/// `repo` を `dest` に bare clone する。
pub fn clone_bare(repo: &str, dest: &Path) -> Result<(), AppError> {
    let dest = dest.to_string_lossy();
    run_git(&["clone", "--bare", "--quiet", "--", repo, &dest]).map(|_| ())
}

/// `repo_dir` のコミットを列挙する。`branch` が `None` のときは全参照が対象。
pub fn list_commits(repo_dir: &Path, branch: Option<&str>) -> Result<Vec<Commit>, AppError> {
    let dir = repo_dir.to_string_lossy();
    let target = branch.unwrap_or("--all");
    let output = run_git(&["-C", &dir, "rev-list", target, "--"])?;
    let commits = parse_commits(&String::from_utf8_lossy(&output.stdout));
    if commits.is_empty() {
        return Err(AppError::EmptyRepository);
    }
    Ok(commits)
}

fn parse_commits(stdout: &str) -> Vec<Commit> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|hash| Commit {
            hash: hash.to_string(),
        })
        .collect()
}

fn run_git(args: &[&str]) -> Result<Output, AppError> {
    let output = Command::new("git").args(args).output().map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            AppError::GitNotFound
        } else {
            AppError::CloneFailed(err.to_string())
        }
    })?;
    if output.status.success() {
        Ok(output)
    } else {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(AppError::CloneFailed(detail))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commits_skips_blank_lines() {
        let commits = parse_commits("aaa\n\nbbb\r\n");
        let hashes: Vec<_> = commits.iter().map(|c| c.hash.as_str()).collect();
        assert_eq!(hashes, ["aaa", "bbb"]);
    }

    #[test]
    fn parse_commits_returns_empty_for_empty_output() {
        assert!(parse_commits("").is_empty());
    }
}
