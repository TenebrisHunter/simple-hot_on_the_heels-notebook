// ============================================================
//  i18n/index.ts — локализация
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================
//  Доступные языки: ru, en, zh.
//  Импорт через ?raw + JSON.parse — работает в Vite 6 без assert.
// ============================================================

import { writable, derived } from 'svelte/store';

import ruRaw from './locales/ru.json?raw';
import enRaw from './locales/en.json?raw';
import zhRaw from './locales/zh.json?raw';

const ru = JSON.parse(ruRaw);
const en = JSON.parse(enRaw);
const zh = JSON.parse(zhRaw);

const locales: Record<string, any> = { ru, en, zh };

export const currentLocale = writable<'ru' | 'en' | 'zh'>('ru');

export const t = derived(currentLocale, ($locale) => {
  const dict = locales[$locale];
  return (key: string, params?: Record<string, string | number>): string => {
    if (!dict) return key;
    const keys = key.split('.');
    let value: any = dict;
    for (const k of keys) value = value?.[k];
    if (typeof value !== 'string') return key;
    if (params) {
      return Object.entries(params).reduce(
        (acc, [k, v]) => acc.replace(`{${k}}`, String(v)), value
      );
    }
    return value;
  };
});