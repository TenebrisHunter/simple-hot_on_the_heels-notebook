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

export async function updateLesson(groupName: string, lesson: Lesson) {
  await saveLesson(groupName, lesson);
  await refreshLessons(groupName);
}

export async function removeLesson(groupName: string, date: string) {
  await deleteLesson(groupName, date);
  await refreshLessons(groupName);
}

/// Пометить несколько занятий как отмеченные/неотмеченные
export async function markLessons(groupName: string, dates: string[], marked: boolean) {
  const list = await loadLessons(groupName);
  for (const lesson of list) {
    if (dates.includes(lesson.date)) {
      (lesson as any).marked = marked;
      await saveLesson(groupName, lesson);
    }
  }
  await refreshLessons(groupName);
}