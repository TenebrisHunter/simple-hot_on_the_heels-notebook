import type { Lesson } from './storage';

/// Формирует HTML-таблицу для вставки в Яндекс.Таблицы
export function copyAllAsHtml(lesson: Lesson): string {
  const rows = lesson.students
    .map(s => {
      const cells = [
        `<td>${escapeHtml(s.name)}</td>`,
        `<td>${s.present ? 'да' : 'нет'}</td>`
      ];
      if (s.reason) cells.push(`<td>${escapeHtml(s.reason)}</td>`);
      return `<tr>${cells.join('')}</tr>`;
    })
    .join('');
  return `<table>${rows}</table>`;
}

/// Формирует HTML только со списком учеников
export function copyStudentsAsHtml(lesson: Lesson): string {
  const rows = lesson.students
    .map(s => `<tr><td>${escapeHtml(s.name)}</td><td>${s.present ? 'да' : 'нет'}</td></tr>`)
    .join('');
  return `<table>${rows}</table>`;
}

/// Обычный текст (на случай, если HTML не сработает)
export function copyAll(lesson: Lesson): string {
  return lesson.students
    .map(s => `${s.name}\t${s.present ? 'да' : 'нет'}${s.reason ? '\t' + s.reason : ''}`)
    .join('\n');
}

export function copyStudents(lesson: Lesson): string {
  return lesson.students
    .map(s => `${s.name}\t${s.present ? 'да' : 'нет'}`)
    .join('\n');
}

export function copyTopic(lesson: Lesson): string {
  return lesson.topic;
}

export function copyMaterials(lesson: Lesson): string {
  return lesson.materials;
}

/// Копирует HTML в буфер (для вставки в Яндекс.Таблицы)
export async function toClipboardHtml(html: string, plain: string): Promise<void> {
  const blob = new Blob([html], { type: 'text/html' });
  const textBlob = new Blob([plain], { type: 'text/plain' });
  const item = new ClipboardItem({
    'text/html': blob,
    'text/plain': textBlob
  });
  await navigator.clipboard.write([item]);
}

export async function toClipboard(text: string): Promise<void> {
  await navigator.clipboard.writeText(text);
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}
