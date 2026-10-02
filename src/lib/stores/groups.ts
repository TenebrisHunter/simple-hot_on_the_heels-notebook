// ============================================================
//  groups.ts — стор групп
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================

import { writable } from 'svelte/store';
import type { Group } from '../utils/storage';
import { loadGroups, saveGroup, deleteGroup, renameGroup } from '../utils/storage';

export const groups = writable<Group[]>([]);

export async function refreshGroups() {
  const list = await loadGroups();
  groups.set(list);
}

export async function addGroup(group: Group) {
  await saveGroup(group);
  await refreshGroups();
}

/// Обновляет группу. Если имя изменилось — переименовывает папку.
export async function updateGroup(oldName: string, group: Group) {
  if (oldName !== group.name) {
    await renameGroup(oldName, group.name);
  }
  await saveGroup(group);
  await refreshGroups();
}

export async function removeGroup(name: string) {
  await deleteGroup(name);
  await refreshGroups();
}