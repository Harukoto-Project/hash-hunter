use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use crate::error::AppError;
use crate::git::api;

/// コミットを列挙する対象のリポジトリ。
/// URL の場合は一時ディレクトリに bare clone し、破棄時にそのディレクトリを削除する。
#[derive(Debug)]
pub struct Workspace {
    path: PathBuf,
    temporary: bool,
}

impl Workspace {
    /// `repo` がローカルパスならそのまま参照し、そうでなければ一時ディレクトリへ clone する。
    pub fn open(repo: &str) -> Result<Self, AppError> {
        api::ensure_git_available()?;

        let local = Path::new(repo);
        if local.exists() {
            return Ok(Self {
                path: local.to_path_buf(),
                temporary: false,
            });
        }

        let workspace = Self {
            path: unique_temp_path(),
            temporary: true,
        };
        api::clone_bare(repo, &workspace.path)?;
        Ok(workspace)
    }

    /// リポジトリのディレクトリを返す。
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        if self.temporary {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

fn unique_temp_path() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    env::temp_dir().join(format!("hash-hunter-{}-{nanos}", process::id()))
}
