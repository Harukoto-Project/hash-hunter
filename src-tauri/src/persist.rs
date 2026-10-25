use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::model::ScanSnapshot;

const FILE_NAME: &str = "last-scan.json";

fn file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|err| err.to_string())?;
    Ok(dir.join(FILE_NAME))
}

/// 直近スキャンをアプリデータへ保存する。
pub fn save(app: &AppHandle, snapshot: &ScanSnapshot) -> Result<(), String> {
    let path = file_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let json = serde_json::to_string_pretty(snapshot).map_err(|err| err.to_string())?;
    fs::write(path, json).map_err(|err| err.to_string())
}

/// 直近スキャンを読む。無ければ `None`。
pub fn load(app: &AppHandle) -> Result<Option<ScanSnapshot>, String> {
    let path = file_path(app)?;
    if !path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let snapshot = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    Ok(Some(snapshot))
}
