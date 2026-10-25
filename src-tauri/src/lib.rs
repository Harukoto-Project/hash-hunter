mod commands;
mod model;
mod persist;

/// Tauri アプリケーションを起動する。
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::list_detectors,
            commands::scan,
            commands::load_last_scan
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
