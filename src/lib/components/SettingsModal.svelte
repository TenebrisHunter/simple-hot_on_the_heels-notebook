<!--
  ============================================================
  SettingsModal.svelte — окно настроек
  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
  Версия: stable&work_2_[v61]
  ============================================================
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { t, currentLocale } from '../i18n';
  import { theme, locale } from '../stores/settings';
  import { escapeKey } from '../utils/escapeAction';
  import { getDataDir, setDataDir, getDefaultDataDir, openFolder } from '../utils/storage';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  export let onClose: () => void;

  let tab: 'about' | 'appearance' | 'language' | 'data' = 'about';
  let dataDir = '';
  let defaultDir = '';
  let changing = false;
  let needsRestart = false;

  onMount(async () => {
    try {
      dataDir = await getDataDir();
      defaultDir = await getDefaultDataDir();
    } catch (e) {
      console.error('Не удалось получить папку данных:', e);
    }
  });

  function setLocale(l: 'ru' | 'en' | 'zh') {
    locale.set(l);
    currentLocale.set(l);
  }

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') {
      changing = true;
      try {
        await setDataDir(selected);
        dataDir = selected;
        needsRestart = true;
      } catch (e) {
        alert('Ошибка: ' + e);
      } finally {
        changing = false;
      }
    }
  }

  async function resetToDefault() {
    changing = true;
    try {
      await setDataDir(defaultDir);
      dataDir = defaultDir;
      needsRestart = true;
    } catch (e) {
      alert('Ошибка: ' + e);
    } finally {
      changing = false;
    }
  }

  async function openInExplorer() {
    try {
      await openFolder(dataDir);
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="overlay" use:escapeKey={onClose}>
  <div class="dialog">
    <div class="header">
      <h2>⚙️ {$t('settings.title')}</h2>
      <button class="close" on:click={onClose}>×</button>
    </div>

    <div class="tabs">
      <button class:active={tab === 'about'} on:click={() => tab = 'about'}>ℹ️ {$t('settings.tab_about')}</button>
      <button class:active={tab === 'appearance'} on:click={() => tab = 'appearance'}>🎨 {$t('settings.tab_appearance')}</button>
      <button class:active={tab === 'language'} on:click={() => tab = 'language'}>🌐 {$t('settings.tab_language')}</button>
      <button class:active={tab === 'data'} on:click={() => tab = 'data'}>📁 {$t('settings.tab_data')}</button>
    </div>

    <div class="content">
      {#if tab === 'about'}
        <h3>simple-hot_on_the_heels-notebook</h3>
        <p>{$t('app.title')}</p>
        <p><strong>{$t('settings.version')}:</strong> stable&work_2_[v61]</p>
        <p><strong>{$t('settings.date')}:</strong> 14.10.2026</p>
        <p><strong>{$t('settings.author')}:</strong> Ключенко М.А.</p>
        <p><strong>{$t('settings.organization')}:</strong> Омск, ОмГТУ, ИБа-261</p>
        <p><strong>{$t('settings.license')}:</strong> MIT</p>
      {:else if tab === 'appearance'}
        <h3>{$t('settings.theme_title')}</h3>
        <div class="radio-group">
          <label class="radio">
            <input type="radio" bind:group={$theme} value="light" />
            ☀️ {$t('settings.theme_light')}
          </label>
          <label class="radio">
            <input type="radio" bind:group={$theme} value="dark" />
            🌙 {$t('settings.theme_dark')}
          </label>
        </div>
      {:else if tab === 'language'}
        <h3>{$t('settings.lang_title')}</h3>
        <div class="radio-group">
          <label class="radio"><input type="radio" checked={$locale === 'ru'} on:change={() => setLocale('ru')} /> 🇷🇺 {$t('settings.lang_ru')}</label>
          <label class="radio"><input type="radio" checked={$locale === 'en'} on:change={() => setLocale('en')} /> 🇬🇧 {$t('settings.lang_en')}</label>
          <label class="radio"><input type="radio" checked={$locale === 'zh'} on:change={() => setLocale('zh')} /> 🇨🇳 {$t('settings.lang_zh')}</label>
        </div>
      {:else if tab === 'data'}
        <h3>{$t('settings.data_title')}</h3>
        <p class="hint">{$t('settings.data_hint')}</p>

        <div class="data-path">
          <code>{dataDir}</code>
        </div>

        <div class="data-buttons">
          <button on:click={pickFolder} disabled={changing}>📁 {$t('settings.data_change')}</button>
          <button on:click={openInExplorer}>🔍 {$t('settings.data_open')}</button>
          <button on:click={resetToDefault} disabled={changing}>↩️ {$t('settings.data_reset')}</button>
        </div>

        {#if needsRestart}
          <p class="warning">⚠️ {$t('settings.data_restart')}</p>
        {/if}
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1200; }
  .dialog { background: var(--bg-card); color: var(--text); padding: 24px; border-radius: 8px; max-width: 650px; width: 90%; max-height: 85vh; display: flex; flex-direction: column; }
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  h2 { margin: 0; font-size: 1.2rem; }
  h3 { margin: 0 0 12px; font-size: 1rem; }
  p { margin: 4px 0; font-size: 0.9rem; color: var(--text-secondary); }
  .hint { font-size: 0.85rem; color: var(--text-muted); margin-bottom: 12px; }
  .tabs { display: flex; gap: 6px; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 8px; flex-wrap: wrap; }
  .tabs button { background: none; border: none; padding: 8px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; color: var(--text-secondary); }
  .tabs button.active { background: var(--accent); color: white; }
  .content { overflow-y: auto; flex: 1; }
  .radio-group { display: flex; flex-direction: column; gap: 8px; }
  .radio { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px; border-radius: 4px; font-size: 0.95rem; }
  .radio:hover { background: var(--bg-hover); }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--text); }

  .data-path {
    background: var(--bg-muted);
    border-radius: 4px;
    padding: 12px;
    margin-bottom: 12px;
    word-break: break-all;
  }
  .data-path code { font-family: monospace; font-size: 0.85rem; color: var(--text); }
  .data-buttons { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 12px; }
  .data-buttons button {
    background: var(--accent); color: white; border: none;
    padding: 8px 14px; border-radius: 4px; cursor: pointer; font-size: 0.85rem;
  }
  .data-buttons button:disabled { opacity: 0.6; cursor: not-allowed; }
  .warning { color: var(--warning); font-size: 0.85rem; padding: 8px; background: var(--bg-muted); border-radius: 4px; }
</style>