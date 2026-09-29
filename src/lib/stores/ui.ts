import { writable } from 'svelte/store';

export const view = writable<'groups' | 'lessons' | 'import' | 'trash'>('groups');
export const selectedGroup = writable<string | null>(null);
export const lessonsGroup = writable<string | null>(null);
export const selectedLesson = writable<any | null>(null);
export const loading = writable<boolean>(false);
export const confirmMessage = writable<string | null>(null);
export const confirmCallback = writable<(() => void) | null>(null);
export const selectedLessons = writable<Set<string>>(new Set());

const MIN_LOADING_MS = 400;

export async function withMinLoading<T>(fn: () => Promise<T>): Promise<T> {
  loading.set(true);
  const start = Date.now();
  try {
    const result = await fn();
    const elapsed = Date.now() - start;
    if (elapsed < MIN_LOADING_MS) {
      await new Promise(r => setTimeout(r, MIN_LOADING_MS - elapsed));
    }
    return result;
  } finally {
    loading.set(false);
  }
}