// ============================================================
//  formatDate.ts — форматирование даты для UI
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
// ============================================================
//  ВАЖНО:
//    - В txt-файлах, JSON, Rust — формат ISO 8601: YYYY-MM-DD.
//    - Это нужно для правильной сортировки (строки сортируются
//      как даты: 2026-09-29 < 2026-10-02).
//    - В UI (список занятий, просмотр) — показываем DD.MM.YYYY.
//      Это привычнее пользователю.
//    - Между ними — эта функция. Меняем ТОЛЬКО отображение,
//      формат хранения НЕ трогаем.
// ============================================================

/// YYYY-MM-DD → DD.MM.YYYY
export function formatDate(iso: string): string {
  if (!iso) return '';
  const parts = iso.split('-');
  if (parts.length !== 3) return iso;
  const [y, m, d] = parts;
  return `${d}.${m}.${y}`;
}

/// YYYY-MM-DD + HH:MM:SS → DD.MM.YYYY HH:MM:SS
export function formatDateTime(date: string, time: string): string {
  return `${formatDate(date)} ${time}`;
}