import { invoke } from '@tauri-apps/api/core';

export interface Student { name: string; present: boolean; reason?: string; }
export interface Group { name: string; students: string[]; }
export interface Lesson { date: string; hours: number; topic: string; materials: string; students: Student[]; marked?: boolean; }

export async function loadGroups(): Promise<Group[]> { return await invoke('load_groups'); }
export async function saveGroup(group: Group): Promise<void> { await invoke('save_group', { group }); }
export async function deleteGroup(name: string): Promise<void> { await invoke('delete_group', { name }); }
export async function loadLessons(groupName: string): Promise<Lesson[]> { return await invoke('load_lessons', { groupName }); }
export async function saveLesson(groupName: string, lesson: Lesson): Promise<void> { await invoke('save_lesson', { groupName, lesson }); }
export async function deleteLesson(groupName: string, date: string): Promise<void> { await invoke('delete_lesson', { groupName, date }); }
export async function importFromFolder(folderPath: string, groupName: string): Promise<number> { return await invoke('import_from_folder', { folderPath, groupName }); }
export async function listTrash(groupName: string): Promise<string[]> { return await invoke('list_trash', { groupName }); }
export async function restoreFromTrash(groupName: string, filename: string): Promise<void> { await invoke('restore_from_trash', { groupName, filename }); }
export async function deleteFromTrash(groupName: string, filename: string): Promise<void> { await invoke('delete_from_trash', { groupName, filename }); }
export async function cleanOldTrash(): Promise<void> { await invoke('clean_old_trash'); }