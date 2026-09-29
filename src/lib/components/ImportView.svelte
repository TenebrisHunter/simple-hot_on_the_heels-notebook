<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { importFromFolder } from '../utils/storage';
  import { groups, refreshGroups } from '../stores/groups';
  import { withMinLoading } from '../stores/ui';
  import { open } from '@tauri-apps/plugin-dialog';

  let folderPath = '';
  let groupName = '';
  let message = '';
  let importing = false;

  onMount(refreshGroups);

  async function selectFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') folderPath = selected;
  }

  async function doImport() {
    if (!folderPath || !groupName) return;
    importing = true;
    try {
      const count = await withMinLoading(() => importFromFolder(folderPath, groupName));
      message = $t('import.success', { count });
      await refreshGroups();
    } catch (e) {
      message = $t('import.error') + ': ' + e;
    }
    importing = false;
  }
</script>

<div class="import-view">
  <h2>📥 {$t('import.title')}</h2>

  <label>
    Папка с txt-файлами:
    <button class="folder-btn" on:click={selectFolder}>📁 {$t('import.select_folder')}</button>
    {#if folderPath}<span class="path">{folderPath}</span>{/if}
  </label>

  <label>
    В какую группу импортировать:
    <select bind:value={groupName}>
      <option value="">— выбери группу —</option>
      {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
    </select>
  </label>

  <button class="import-btn" on:click={doImport} disabled={importing || !folderPath || !groupName}>
    {#if importing}<span class="loader"></span> Импорт...{:else}📥 {$t('import.import')}{/if}
  </button>

  {#if message}<p class="message">{message}</p>{/if}
</div>

<style>
  .import-view { padding: 16px; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label { display: block; margin-bottom: 16px; font-size: 0.9rem; color: #555; }
  select { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; }
  .folder-btn { background: #eee; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; margin-top: 4px; }
  .path { display: block; font-size: 0.8rem; color: #999; margin-top: 4px; }
  .import-btn { background: #4a90d9; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; display: inline-flex; align-items: center; gap: 8px; }
  .import-btn:disabled { opacity: 0.6; cursor: not-allowed; }
  .loader { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 0.6s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .message { margin-top: 16px; color: #5cb85c; }
</style>