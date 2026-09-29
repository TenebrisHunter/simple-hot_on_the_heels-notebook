<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { groups } from '../stores/groups';
  import { confirmMessage, confirmCallback } from '../stores/ui';
  import { listTrash, restoreFromTrash, deleteFromTrash } from '../utils/storage';

  let trashGroup = '';
  let files: string[] = [];
  let loading = false;

  $: if (trashGroup) loadTrash();

  async function loadTrash() {
    loading = true;
    files = await listTrash(trashGroup);
    loading = false;
  }

  async function restore(filename: string) {
    await restoreFromTrash(trashGroup, filename);
    await loadTrash();
  }

  function askDelete(filename: string) {
    $confirmMessage = 'Удалить файл навсегда?';
    $confirmCallback = async () => {
      await deleteFromTrash(trashGroup, filename);
      $confirmMessage = null;
      $confirmCallback = null;
      await loadTrash();
    };
  }
</script>

<div class="trash-view">
  <h2>Корзина</h2>

  <label>
    Группа:
    <select bind:value={trashGroup}>
      <option value="">— выбери —</option>
      {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
    </select>
  </label>

  {#if loading}
    <p>Загрузка...</p>
  {:else if !trashGroup}
    <p class="empty">Выбери группу</p>
  {:else if files.length === 0}
    <p class="empty">Корзина пуста</p>
  {:else}
    {#each files as file}
      <div class="trash-item">
        <span>{file}</span>
        <div class="actions">
          <button class="restore" on:click={() => restore(file)}>Восстановить</button>
          <button class="delete" on:click={() => askDelete(file)}>Удалить</button>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .trash-view { padding: 16px; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label { display: block; margin-bottom: 16px; font-size: 0.9rem; color: #555; }
  select { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid #ccc; border-radius: 4px; }
  .empty { color: #999; text-align: center; padding: 32px; }
  .trash-item { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; border: 1px solid #eee; border-radius: 6px; margin-bottom: 8px; }
  .actions { display: flex; gap: 4px; }
  .restore { background: #5cb85c; color: white; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .delete { background: #d9534f; color: white; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
</style>