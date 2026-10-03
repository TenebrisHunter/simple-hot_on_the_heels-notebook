// ============================================================
//  commands.rs — Tauri-команды (мост между UI и storage.rs)
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================
//  Что делает:
//    - Регистрирует публичные команды для вызова из Svelte.
//    - Каждая команда — тонкая обёртка над storage.rs.
//  Как добавить новую:
//    1. Написать функцию в storage.rs.
//    2. Добавить #[tauri::command] здесь.
//    3. Зарегистрировать в lib.rs (invoke_handler).
// ============================================================

use crate::storage::{self, Group, Lesson};

#[tauri::command]
pub fn load_groups() -> Result<Vec<Group>, String> { storage::load_groups() }

#[tauri::command]
pub fn save_group(group: Group) -> Result<(), String> { storage::save_group(&group) }

#[tauri::command]
pub fn delete_group(name: String) -> Result<(), String> { storage::delete_group(&name) }

#[tauri::command]
pub fn load_lessons(group_name: String) -> Result<Vec<Lesson>, String> { storage::load_lessons(&group_name) }

#[tauri::command]
pub fn lesson_exists(group_name: String, date: String) -> Result<bool, String> { storage::lesson_exists(&group_name, &date) }

#[tauri::command]
pub fn save_lesson(group_name: String, lesson: Lesson, overwrite: bool) -> Result<String, String> { storage::save_lesson(&group_name, &lesson, overwrite) }

#[tauri::command]
pub fn delete_lesson(group_name: String, file_id: String) -> Result<(), String> { storage::delete_lesson(&group_name, &file_id) }

#[tauri::command]
pub fn delete_lessons(group_name: String, file_ids: Vec<String>) -> Result<usize, String> { storage::delete_lessons(&group_name, file_ids) }

#[tauri::command]
pub fn import_from_folder(folder_path: String, group_name: String) -> Result<usize, String> { storage::import_from_folder(&folder_path, &group_name) }

#[tauri::command]
pub fn list_trash_lessons(group_name: String) -> Result<Vec<String>, String> { storage::list_trash_lessons(&group_name) }

#[tauri::command]
pub fn list_trash_groups() -> Result<Vec<String>, String> { storage::list_trash_groups() }

#[tauri::command]
pub fn restore_trash_lesson(group_name: String, file_id: String) -> Result<(), String> { storage::restore_trash_lesson(&group_name, &file_id) }

#[tauri::command]
pub fn restore_trash_group(trash_name: String) -> Result<(), String> { storage::restore_trash_group(&trash_name) }

#[tauri::command]
pub fn delete_trash_lesson(group_name: String, file_id: String) -> Result<(), String> { storage::delete_trash_lesson(&group_name, &file_id) }

#[tauri::command]
pub fn delete_trash_group(trash_name: String) -> Result<(), String> { storage::delete_trash_group(&trash_name) }

#[tauri::command]
pub fn clean_old_trash() -> Result<(), String> { storage::clean_old_trash() }

#[tauri::command]
pub fn toggle_mark(group_name: String, lesson: Lesson) -> Result<(), String> { storage::toggle_mark(&group_name, &lesson) }
#[tauri::command]
pub fn rename_group(old_name: String, new_name: String) -> Result<(), String> {
    storage::rename_group(&old_name, &new_name)
}

#[tauri::command]
pub fn has_trash_for_group(group_name: String) -> Result<bool, String> {
    storage::has_trash_for_group(&group_name)
}
