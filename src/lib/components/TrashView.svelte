<!--
  ============================================================
  TrashView.svelte — корзина (занятия + группы)
  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { groups, refreshGroups } from '../stores/groups';
  import { confirmMessage, confirmCallback, withMinLoading } from '../stores/ui';
  import {
    listTrashLessons, listTrashGroups,
    restoreTrashLesson, restoreTrashGroup,
    deleteTrashLesson, deleteTrashGroup
  } from '../utils/storage';
  import LoadingSpinner from './LoadingSpinner.svelte';

  let tab: 'lessons' | 'groups' = 'lessons';
  let trashGroup = '';
  let lessonsFiles: string[] = [];
  let groupsFiles: string[] = [];
  let loading = false;
  let busyId: string | null = null;

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
    busyId = fileId;
    try {
      await withMinLoading(async () => {
        await restoreTrashLesson(trashGroup, fileId);
        await loadLessons();
      });
    } finally {
      busyId = null;
    }
  }

  async function restoreG(trashName: string) {
    busyId = trashName;
    try {
      await withMinLoading(async () => {
        await restoreTrashGroup(trashName);
        await loadGroups();
        await refreshGroups();
      });
    } finally {
      busyId = null;
    }
  }

  function askDeleteL(fileId: string) {
    $confirmMessage = $t('trash.confirm_delete_lesson');
    $confirmCallback = async () => {
      $confirmMessage = null;
      $confirmCallback = null;
      busyId = fileId;
      try {
        await withMinLoading(async () => {
          await deleteTrashLesson(trashGroup, fileId);
          await loadLessons();
        });
      } finally {
        busyId = null;
      }
    };
  }

  function askDeleteG(trashName: string) {
    $confirmMessage = $t('trash.confirm_delete_group', { name: cleanGroupName(trashName) });
    $confirmCallback = async () => {
      $confirmMessage = null;
      $confirmCallback = null;
      busyId = trashName;
      try {
        await withMinLoading(async () => {
          await deleteTrashGroup(trashName);
          await loadGroups();
        });
      } finally {
        busyId = null;
      }
    };
  }

  function cleanGroupName(trashName: string): string {
    const parts = trashName.split('_');
    if (parts.length >= 2 && /^\d+$/.test(parts[parts.length - 1])) parts.pop();
    return parts.join('_');
  }
</script>

<div class="trash-view">
  <h2>🗑️ {$t('trash.title')}</h2>
  <p class="hint">{$t('trash.hint')}</p>

  <div class="tabs">
    <button class:active={tab === 'lessons'} on:click={() => tab = 'lessons'}>📚 {$t('trash.tab_lessons')}</button>
    <button class:active={tab === 'groups'} on:click={() => tab = 'groups'}>👥 {$t('trash.tab_groups')}</button>
  </div>

  {#if tab === 'lessons'}
    <label>
      {$t('trash.group')}:
      <select bind:value={trashGroup}>
        <option value="">{$t('trash.choose')}</option>
        {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
      </select>
    </label>

    {#if loading}<p>{$t('trash.loading')}</p>
    {:else if !trashGroup}<p class="empty">{$t('trash.choose_group')}</p>
    {:else if lessonsFiles.length === 0}<p class="empty">{$t('trash.empty')}</p>
    {:else}
      {#each lessonsFiles as file}
        <div class="item">
          <span>{file}</span>
          <div class="actions">
            <button class="restore" on:click={() => restoreL(file)} disabled={busyId === file}>
              {#if busyId === file}<LoadingSpinner active size={14} color="#2e7d32" />{:else}↩️ {$t('trash.restore')}{/if}
            </button>
            <button class="delete" on:click={() => askDeleteL(file)} disabled={busyId === file}>
              {#if busyId === file}<LoadingSpinner active size={14} color="#d9534f" />{:else}🗑️ {$t('trash.delete_forever')}{/if}
            </button>
          </div>
        </div>
      {/each}
    {/if}
  {:else}
    {#if loading}<p>{$t('trash.loading')}</p>
    {:else if groupsFiles.length === 0}<p class="empty">{$t('trash.empty')}</p>
    {:else}
      {#each groupsFiles as file}
        <div class="item">
          <span>{cleanGroupName(file)}</span>
          <div class="actions">
            <button class="restore" on:click={() => restoreG(file)} disabled={busyId === file}>
              {#if busyId === file}<LoadingSpinner active size={14} color="#2e7d32" />{:else}↩️ {$t('trash.restore')}{/if}
            </button>
            <button class="delete" on:click={() => askDeleteG(file)} disabled={busyId === file}>
              {#if busyId === file}<LoadingSpinner active size={14} color="#d9534f" />{:else}🗑️ {$t('trash.delete_forever')}{/if}
            </button>
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
  .restore, .delete { border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; display: inline-flex; align-items: center; gap: 6px; min-width: 160px; justify-content: center; }
  .restore { background: #e8f5e9; color: #2e7d32; }
  .delete { background: #fdecea; color: #d9534f; }
  .restore:disabled, .delete:disabled { opacity: 0.7; cursor: wait; }
</style>