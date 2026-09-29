import { writable, derived } from 'svelte/store';
import ru from './locales/ru.json';
import en from './locales/en.json';

const locales = { ru, en };
export const currentLocale = writable<'ru' | 'en'>('ru');

export const t = derived(currentLocale, ($locale) => {
  const dict = locales[$locale];
  return (key: string, params?: Record<string, string | number>): string => {
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
