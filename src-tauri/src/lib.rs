// ============================================================
//  simple-hot_on_the_heels-notebook — ядро
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
//  Версия: stable&work_2_[v61]
// ============================================================

mod config;
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
            commands::rename_group,
            commands::has_trash_for_group,
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
            commands::get_data_dir,
            commands::set_data_dir,
            commands::is_data_dir_configured,
            commands::get_default_data_dir,
            commands::open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}