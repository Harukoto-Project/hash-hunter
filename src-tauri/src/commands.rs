use tauri::AppHandle;

use crate::model::{CommandError, HitDto, RankedHashDto, ScanInput, ScanSnapshot};
use crate::persist;

/// 登録順の検出器 ID 一覧を返す。
#[tauri::command]
pub fn list_detectors() -> Vec<String> {
    hash_hunter::detect::list_ids()
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// リポジトリをスキャンし、結果を保存して返す。
#[tauri::command]
pub async fn scan(app: AppHandle, input: ScanInput) -> Result<ScanSnapshot, CommandError> {
    let snapshot = tauri::async_runtime::spawn_blocking(move || run_scan(input))
        .await
        .expect("scan worker")?;
    let _ = persist::save(&app, &snapshot);
    Ok(snapshot)
}

/// 保存済みの直近スキャンを返す。
#[tauri::command]
pub fn load_last_scan(app: AppHandle) -> Option<ScanSnapshot> {
    persist::load(&app).ok().flatten()
}

fn run_scan(input: ScanInput) -> Result<ScanSnapshot, CommandError> {
    let scored = hash_hunter::scan::collect(
        &input.repo,
        input.branch.as_deref(),
        &input.only,
        &input.exclude,
    )?;
    let ranked = hash_hunter::report::rank(&scored, input.top, input.min_score);
    let ranking = ranked.into_iter().map(to_ranked_dto).collect();
    Ok(ScanSnapshot { input, ranking })
}

fn to_ranked_dto(item: hash_hunter::detect::ScoredHash) -> RankedHashDto {
    let mut hits = item.hits;
    hits.sort_by(|a, b| {
        a.range
            .start
            .cmp(&b.range.start)
            .then_with(|| a.range.end.cmp(&b.range.end))
            .then_with(|| a.detector_id.cmp(b.detector_id))
    });
    RankedHashDto {
        hash: item.hash,
        score: item.score,
        hits: hits
            .into_iter()
            .map(|hit| HitDto {
                detector_id: hit.detector_id.to_string(),
                start: hit.range.start,
                end: hit.range.end,
                score: hit.score,
            })
            .collect(),
    }
}
