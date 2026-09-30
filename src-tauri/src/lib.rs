// ============================================================
//  simple-hot_on_the_heels-notebook — ядро
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
//  Версия: stable&work_1_[v50]
// ============================================================

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
            commands::lesson_exists,
            commands::save_lesson,
            commands::delete_lesson,
            commands::delete_lessons,
            commands::import_from_folder,
            commands::list_trash_lessons,
            commands::list_trash_groups,
            commands::restore_trash_lesson,
            commands::restore_trash_group,
            commands::delete_trash_lesson,
            commands::delete_trash_group,
            commands::clean_old_trash,
            commands::toggle_mark,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}