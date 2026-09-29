import { writable } from 'svelte/store';

export const view = writable<'groups' | 'lessons' | 'import' | 'trash'>('groups');
export const selectedGroup = writable<string | null>(null);
export const lessonsGroup = writable<string | null>(null);
export const selectedLesson = writable<any | null>(null);
export const loading = writable<boolean>(false);
export const confirmMessage = writable<string | null>(null);
export const confirmCallback = writable<(() => void) | null>(null);
export const selectedLessons = writable<Set<string>>(new Set());