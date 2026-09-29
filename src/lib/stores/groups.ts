import { writable } from 'svelte/store';
import type { Group } from '../utils/storage';
import { loadGroups, saveGroup, deleteGroup } from '../utils/storage';

export const groups = writable<Group[]>([]);

export async function refreshGroups() {
  const list = await loadGroups();
  groups.set(list);
}

export async function addGroup(group: Group) {
  await saveGroup(group);
  await refreshGroups();
}

export async function updateGroup(group: Group) {
  await saveGroup(group);
  await refreshGroups();
}

export async function removeGroup(name: string) {
  await deleteGroup(name);
  await refreshGroups();
}