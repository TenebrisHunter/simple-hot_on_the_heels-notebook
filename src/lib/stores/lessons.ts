import { writable } from 'svelte/store';
import type { Lesson } from '../utils/storage';
import { loadLessons, saveLesson, deleteLesson } from '../utils/storage';

export const lessons = writable<Lesson[]>([]);

export async function refreshLessons(groupName: string) {
  const list = await loadLessons(groupName);
  lessons.set(list);
}

export async function addLesson(groupName: string, lesson: Lesson) {
  await saveLesson(groupName, lesson);
  await refreshLessons(groupName);
}

export async function removeLesson(groupName: string, date: string) {
  await deleteLesson(groupName, date);
  await refreshLessons(groupName);
}
