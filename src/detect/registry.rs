use crate::detect::detectors::{
    Ascending, Descending, DigitsOnly, HexWord, LeadingZeros, LettersOnly, Lucky, Palindrome,
    RepeatPattern, RepeatRun, TrailingZeros,
};
use crate::detect::model::Detector;
use crate::error::AppError;

/// 登録済みの全検出器を返す。新しい検出器はここに 1 行足す。
pub fn all() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(LeadingZeros),
        Box::new(TrailingZeros),
        Box::new(RepeatRun),
        Box::new(Ascending),
        Box::new(Descending),
        Box::new(DigitsOnly),
        Box::new(LettersOnly),
        Box::new(Palindrome),
        Box::new(RepeatPattern),
        Box::new(HexWord),
        Box::new(Lucky),
    ]
}

/// `only` / `exclude` の ID で絞り込んだ検出器を返す。
/// `only` が空なら全検出器が対象。存在しない ID があれば `unknown_detector` エラーにする。
pub fn select(only: &[String], exclude: &[String]) -> Result<Vec<Box<dyn Detector>>, AppError> {
    let detectors = all();
    for id in only.iter().chain(exclude) {
        if !detectors.iter().any(|d| d.id() == id) {
            return Err(AppError::UnknownDetector(id.clone()));
        }
    }
    Ok(detectors
        .into_iter()
        .filter(|d| only.is_empty() || only.iter().any(|id| id == d.id()))
        .filter(|d| !exclude.iter().any(|id| id == d.id()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(detectors: &[Box<dyn Detector>]) -> Vec<&'static str> {
        detectors.iter().map(|d| d.id()).collect()
    }

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn selects_all_when_no_filter() {
        let selected = select(&[], &[]).unwrap();
        assert_eq!(selected.len(), all().len());
    }

    #[test]
    fn only_limits_detectors() {
        let selected = select(&strings(&["repeat-run"]), &[]).unwrap();
        assert_eq!(ids(&selected), ["repeat-run"]);
    }

    #[test]
    fn exclude_removes_detectors() {
        let selected = select(&[], &strings(&["repeat-run"])).unwrap();
        assert!(!ids(&selected).contains(&"repeat-run"));
        assert_eq!(selected.len(), all().len() - 1);
    }

    #[test]
    fn unknown_id_is_an_error() {
        let result = select(&strings(&["nope"]), &[]);
        assert!(matches!(result, Err(AppError::UnknownDetector(id)) if id == "nope"));
    }
}
