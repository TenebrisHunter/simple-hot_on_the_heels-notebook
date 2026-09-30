// ============================================================
//  settings.ts — пользовательские настройки
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================
//  Что хранит:
//    - theme         — 'light' | 'dark'
//    - locale        — 'ru' | 'en'
//  Сохраняется в localStorage, применяется автоматически.
// ============================================================

import { writable } from 'svelte/store';

const KEY_THEME = 'sport-diary:theme';
const KEY_LOCALE = 'sport-diary:locale';

function loadFromStorage<T extends string>(key: string, fallback: T): T {
  if (typeof localStorage === 'undefined') return fallback;
  return (localStorage.getItem(key) as T) || fallback;
}

export const theme = writable<'light' | 'dark'>(loadFromStorage(KEY_THEME, 'light'));
export const locale = writable<'ru' | 'en'>(loadFromStorage(KEY_LOCALE, 'ru'));

theme.subscribe(value => {
  if (typeof localStorage !== 'undefined') localStorage.setItem(KEY_THEME, value);
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', value);
  }
});

locale.subscribe(value => {
  if (typeof localStorage !== 'undefined') localStorage.setItem(KEY_LOCALE, value);
});