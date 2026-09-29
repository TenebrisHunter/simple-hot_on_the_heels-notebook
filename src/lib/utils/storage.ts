// ============================================================
//  storage.ts — обёртка над Tauri-командами
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================
import { invoke } from '@tauri-apps/api/core';

export interface Student { name: string; present: boolean; reason?: string; }
export interface Group { name: string; students: string[]; default_hours: number; }
export interface Lesson {
  date: string;
  time: string;
  hours: number;
  topic: string;
  materials: string;
  students: Student[];
  marked?: boolean;
  file_id?: string;
}

export async function loadGroups(): Promise<Group[]> { return await invoke('load_groups'); }
export async function saveGroup(group: Group): Promise<void> { await invoke('save_group', { group }); }
export async function deleteGroup(name: string): Promise<void> { await invoke('delete_group', { name }); }
export async function loadLessons(groupName: string): Promise<Lesson[]> { return await invoke('load_lessons', { groupName }); }
export async function lessonExists(groupName: string, date: string): Promise<boolean> { return await invoke('lesson_exists', { groupName, date }); }
export async function saveLesson(groupName: string, lesson: Lesson, overwrite: boolean): Promise<string> {
  return await invoke('save_lesson', { groupName, lesson, overwrite });
}
export async function deleteLesson(groupName: string, fileId: string): Promise<void> { await invoke('delete_lesson', { groupName, fileId }); }
export async function deleteLessons(groupName: string, fileIds: string[]): Promise<number> {
  return await invoke('delete_lessons', { groupName, fileIds });
}
export async function importFromFolder(folderPath: string, groupName: string): Promise<number> {
  return await invoke('import_from_folder', { folderPath, groupName });
}
export async function listTrashLessons(groupName: string): Promise<string[]> { return await invoke('list_trash_lessons', { groupName }); }
export async function listTrashGroups(): Promise<string[]> { return await invoke('list_trash_groups'); }
export async function restoreTrashLesson(groupName: string, fileId: string): Promise<void> {
  await invoke('restore_trash_lesson', { groupName, fileId });
}
export async function restoreTrashGroup(trashName: string): Promise<void> { await invoke('restore_trash_group', { trashName }); }
export async function deleteTrashLesson(groupName: string, fileId: string): Promise<void> {
  await invoke('delete_trash_lesson', { groupName, fileId });
}
export async function deleteTrashGroup(trashName: string): Promise<void> { await invoke('delete_trash_group', { trashName }); }
export async function cleanOldTrash(): Promise<void> { await invoke('clean_old_trash'); }
export async function toggleMark(groupName: string, lesson: Lesson): Promise<void> {
  await invoke('toggle_mark', { groupName, lesson });
}