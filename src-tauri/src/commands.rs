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
pub fn save_lesson(group_name: String, lesson: Lesson) -> Result<(), String> { storage::save_lesson(&group_name, &lesson) }

#[tauri::command]
pub fn delete_lesson(group_name: String, date: String) -> Result<(), String> { storage::delete_lesson(&group_name, &date) }

#[tauri::command]
pub fn import_from_folder(folder_path: String, group_name: String) -> Result<usize, String> { storage::import_from_folder(&folder_path, &group_name) }

#[tauri::command]
pub fn list_trash(group_name: String) -> Result<Vec<String>, String> { storage::list_trash(&group_name) }

#[tauri::command]
pub fn restore_from_trash(group_name: String, filename: String) -> Result<(), String> { storage::restore_from_trash(&group_name, &filename) }

#[tauri::command]
pub fn delete_from_trash(group_name: String, filename: String) -> Result<(), String> { storage::delete_from_trash(&group_name, &filename) }

#[tauri::command]
pub fn clean_old_trash() -> Result<(), String> { storage::clean_old_trash() }