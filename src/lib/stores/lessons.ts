import { writable } from 'svelte/store';
import type { Lesson } from '../utils/storage';
import { loadLessons, saveLesson, deleteLesson, deleteLessons } from '../utils/storage';

export const lessons = writable<Lesson[]>([]);

export async function refreshLessons(groupName: string) {
  lessons.set(await loadLessons(groupName));
}

export async function addLesson(groupName: string, lesson: Lesson, overwrite: boolean): Promise<string> {
  const id = await saveLesson(groupName, lesson, overwrite);
  await refreshLessons(groupName);
  return id;
}

export async function toggleMark(groupName: string, lesson: Lesson) {
  const updated = { ...lesson, marked: !lesson.marked };
  await saveLesson(groupName, updated, true);
  await refreshLessons(groupName);
}

export async function removeLesson(groupName: string, fileId: string) {
  await deleteLesson(groupName, fileId);
  await refreshLessons(groupName);
}

export async function removeLessons(groupName: string, fileIds: string[]) {
  await deleteLessons(groupName, fileIds);
  await refreshLessons(groupName);
}