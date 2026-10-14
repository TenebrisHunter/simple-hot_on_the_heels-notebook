// ============================================================
//  simple-hot_on_the_heels-notebook — ядро
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
//  Версия: stable&work_2_[v61]
// ============================================================

mod config;
mod storage;
mod commands;

use tauri::Emitter;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // --- Меню трея ---
            let open_item = MenuItem::with_id(app, "open", "Открыть", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "Настройки", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &settings_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Дневник занятий")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "open" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "settings" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.emit("open-settings", ());
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // --- Перехват закрытия окна ---
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        if config::is_tray_enabled() {
                            api.prevent_close();
                            let _ = window_clone.hide();
                        }
                    }
                });
            }

            Ok(())
        })
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
            commands::get_tray_enabled,
            commands::set_tray_enabled,
            commands::get_notifications_enabled,
            commands::set_notifications_enabled,
            commands::get_notification_time,
            commands::set_notification_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}