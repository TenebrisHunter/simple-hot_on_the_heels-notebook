mod storage;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_groups,
            commands::save_group,
            commands::delete_group,
            commands::load_lessons,
            commands::save_lesson,
            commands::delete_lesson,
            commands::import_from_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}