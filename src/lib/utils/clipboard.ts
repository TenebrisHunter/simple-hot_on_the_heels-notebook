import type { Lesson } from './storage';

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

export async function toClipboard(text: string): Promise<void> {
  await navigator.clipboard.writeText(text);
}
