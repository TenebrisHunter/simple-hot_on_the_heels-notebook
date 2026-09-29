<script lang="ts">
  import { t } from '../i18n';
  import { importFromFolder } from '../utils/storage';
  import { refreshGroups } from '../stores/groups';
  import { open } from '@tauri-apps/plugin-dialog';

  let folderPath = '';
  let groupName = '';
  let message = '';
  let importing = false;

  async function selectFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') folderPath = selected;
  }

  async function doImport() {
    if (!folderPath || !groupName) return;
    importing = true;
    try {
      const count = await importFromFolder(folderPath, groupName);
      message = $t('import.success', { count });
      await refreshGroups();
    } catch (e) {
      message = $t('import.error') + ': ' + e;
    }
    importing = false;
  }
</script>

<div class="import-view">
  <h2>{$t('import.title')}</h2>

  <label>
    Папка с txt-файлами:
    <button on:click={selectFolder}>{$t('import.select_folder')}</button>
    {#if folderPath}<span class="path">{folderPath}</span>{/if}
  </label>

  <label>
    В какую группу импортировать:
    <input bind:value={groupName} placeholder="Группа А" />
  </label>

  <button class="import-btn" on:click={doImport} disabled={importing}>
    {importing ? $t('common.loading') : $t('import.import')}
  </button>

  {#if message}
    <p class="message">{message}</p>
  {/if}
</div>

<style>
  .import-view { padding: 16px; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label { display: block; margin-bottom: 16px; font-size: 0.9rem; color: #555; }
  input {
    width: 100%; padding: 8px; margin-top: 4px;
    border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;
  }
  .path { display: block; font-size: 0.8rem; color: #999; margin-top: 4px; }
  .import-btn {
    background: #4a90d9; color: white; border: none;
    padding: 10px 20px; border-radius: 4px; cursor: pointer;
  }
  .message { margin-top: 16px; color: #5cb85c; }
</style>
