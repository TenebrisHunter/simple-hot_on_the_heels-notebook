<script lang="ts">
  import { t } from '../i18n';
  import { groups, refreshGroups } from '../stores/groups';
  import { confirmMessage, confirmCallback } from '../stores/ui';
  import {
    listTrashLessons, listTrashGroups,
    restoreTrashLesson, restoreTrashGroup,
    deleteTrashLesson, deleteTrashGroup
  } from '../utils/storage';

  let tab: 'lessons' | 'groups' = 'lessons';
  let trashGroup = '';
  let lessonsFiles: string[] = [];
  let groupsFiles: string[] = [];
  let loading = false;

  $: if (trashGroup && tab === 'lessons') loadLessons();
  $: if (tab === 'groups') loadGroups();

  async function loadLessons() {
    loading = true;
    lessonsFiles = await listTrashLessons(trashGroup);
    loading = false;
  }

  async function loadGroups() {
    loading = true;
    groupsFiles = await listTrashGroups();
    loading = false;
  }

  async function restoreL(fileId: string) {
    await restoreTrashLesson(trashGroup, fileId);
    await loadLessons();
  }

  async function restoreG(trashName: string) {
    await restoreTrashGroup(trashName);
    await loadGroups();
    await refreshGroups();
  }

  function askDeleteL(fileId: string) {
    $confirmMessage = `Удалить запись навсегда?`;
    $confirmCallback = async () => {
      await deleteTrashLesson(trashGroup, fileId);
      $confirmMessage = null;
      $confirmCallback = null;
      await loadLessons();
    };
  }

  function askDeleteG(trashName: string) {
    $confirmMessage = `Удалить группу «${cleanGroupName(trashName)}» навсегда?`;
    $confirmCallback = async () => {
      await deleteTrashGroup(trashName);
      $confirmMessage = null;
      $confirmCallback = null;
      await loadGroups();
    };
  }

  /// Убирает timestamp из имени группы для отображения
  function cleanGroupName(trashName: string): string {
    const parts = trashName.split('_');
    if (parts.length >= 2 && /^\d+$/.test(parts[parts.length - 1])) {
      parts.pop();
    }
    return parts.join('_');
  }
</script>

<div class="trash-view">
  <h2>🗑️ Корзина</h2>
  <p class="hint">Записи хранятся 30 дней, потом удаляются автоматически.</p>

  <div class="tabs">
    <button class:active={tab === 'lessons'} on:click={() => tab = 'lessons'}>📚 Занятия</button>
    <button class:active={tab === 'groups'} on:click={() => tab = 'groups'}>👥 Группы</button>
  </div>

  {#if tab === 'lessons'}
    <label>
      Группа:
      <select bind:value={trashGroup}>
        <option value="">— выбери —</option>
        {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
      </select>
    </label>

    {#if loading}<p>Загрузка...</p>
    {:else if !trashGroup}<p class="empty">Выбери группу</p>
    {:else if lessonsFiles.length === 0}<p class="empty">Пусто</p>
    {:else}
      {#each lessonsFiles as file}
        <div class="item">
          <span>{file}</span>
          <div class="actions">
            <button class="restore" on:click={() => restoreL(file)}>↩️ Восстановить</button>
            <button class="delete" on:click={() => askDeleteL(file)}>🗑️ Удалить навсегда</button>
          </div>
        </div>
      {/each}
    {/if}
  {:else}
    {#if loading}<p>Загрузка...</p>
    {:else if groupsFiles.length === 0}<p class="empty">Пусто</p>
    {:else}
      {#each groupsFiles as file}
        <div class="item">
          <span>{cleanGroupName(file)}</span>
          <div class="actions">
            <button class="restore" on:click={() => restoreG(file)}>↩️ Восстановить</button>
            <button class="delete" on:click={() => askDeleteG(file)}>🗑️ Удалить навсегда</button>
          </div>
        </div>
      {/each}
    {/if}
  {/if}
</div>

<style>
  .trash-view { padding: 16px; }
  h2 { margin: 0 0 8px; font-size: 1.1rem; }
  .hint { color: #999; font-size: 0.85rem; margin: 0 0 16px; }
  .tabs { display: flex; gap: 8px; margin-bottom: 16px; }
  .tabs button { background: #eee; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }
  .tabs button.active { background: #4a90d9; color: white; }
  label { display: block; margin-bottom: 16px; font-size: 0.9rem; color: #555; }
  select { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid #ccc; border-radius: 4px; }
  .empty { color: #999; text-align: center; padding: 32px; }
  .item { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; border: 1px solid #eee; border-radius: 6px; margin-bottom: 8px; }
  .actions { display: flex; gap: 8px; }
  .restore { background: #e8f5e9; color: #2e7d32; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .delete { background: #fdecea; color: #d9534f; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
</style>