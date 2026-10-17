<!--
  ============================================================
  SettingsModal.svelte — окно настроек
  втор: люченко .. (мск, мТ, а-261)
  ерсия: stable&work_2_[v61]
  ============================================================
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { t, currentLocale } from '../i18n';
  import { theme, locale } from '../stores/settings';
  import { escapeKey } from '../utils/escapeAction';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import {
    getDataDir, setDataDir, getDefaultDataDir, openFolder,
    getReminders, saveReminders, getTrayEnabled, setTrayEnabled,
    type Reminder
  } from '../utils/storage';

  export let onClose: () => void;

  let tab: 'about' | 'appearance' | 'language' | 'data' | 'reminders' = 'about';

  // --- апка данных ---
  let dataDir = '';
  let defaultDir = '';
  let changing = false;
  let needsRestart = false;

  // --- Трей ---
  let trayEnabled = false;
  let trayChanged = false;

  // --- GitHub ---
  const githubUrl = 'https://github.com/TenebrisHunter/simple-hot_on_the_heels-notebook';
  let githubCopied = false;

  // --- апоминания ---
  let reminders: Reminder[] = [];

  onMount(async () => {
    try {
      dataDir = await getDataDir();
      defaultDir = await getDefaultDataDir();
      trayEnabled = await getTrayEnabled();
      reminders = await getReminders();
    } catch (e) {
      console.error('шибка загрузки настроек:', e);
    }
  });

  function setLocale(l: 'ru' | 'en' | 'zh') {
    locale.set(l);
    currentLocale.set(l);
  }

  // --- апка данных ---
  async function pickFolder() {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') {
      changing = true;
      try {
        await setDataDir(selected);
        dataDir = selected;
        needsRestart = true;
      } catch (e) {
        alert('шибка: ' + e);
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
      alert('шибка: ' + e);
    } finally {
      changing = false;
    }
  }

  async function openInExplorer() {
    try { await openFolder(dataDir); } catch (e) { console.error(e); }
  }

  // --- GitHub ---
  async function copyGithubUrl() {
    try {
      await navigator.clipboard.writeText(githubUrl);
      githubCopied = true;
      setTimeout(() => githubCopied = false, 1500);
    } catch (e) { console.error(e); }
  }

  async function openGithubUrl() {
    try { await openUrl(githubUrl); } catch (e) { console.error(e); }
  }

  // --- Трей ---
  async function toggleTray() {
    trayEnabled = !trayEnabled;
    await setTrayEnabled(trayEnabled);
    trayChanged = true;
  }

  // --- апоминания ---
  function newId(): string {
    return Date.now().toString(36) + Math.random().toString(36).slice(2, 6);
  }

  async function addReminder() {
    const r: Reminder = {
      id: newId(),
      time: '19:00',
      days: [1, 2, 3, 4, 5, 6, 7],
      enabled: true,
      text: ''
    };
    reminders = [...reminders, r];
    await saveReminders(reminders);
  }

  async function updateReminder(id: string, patch: Partial<Reminder>) {
    reminders = reminders.map(r => r.id === id ? { ...r, ...patch } : r);
    await saveReminders(reminders);
  }

  async function toggleDay(id: string, day: number) {
    const r = reminders.find(x => x.id === id);
    if (!r) return;
    const days = r.days.includes(day)
      ? r.days.filter(d => d !== day)
      : [...r.days, day].sort((a, b) => a - b);
    await updateReminder(id, { days });
  }

  async function deleteReminder(id: string) {
    reminders = reminders.filter(r => r.id !== id);
    await saveReminders(reminders);
  }

  const dayKeys = ['day_1', 'day_2', 'day_3', 'day_4', 'day_5', 'day_6', 'day_7'];
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
      <button class:active={tab === 'reminders'} on:click={() => tab = 'reminders'}>🔔 {$t('settings.tab_reminders')}</button>
    </div>

    <div class="content">
      {#if tab === 'about'}
        <h3>simple-hot_on_the_heels-notebook</h3>
        <p>{$t('app.title')}</p>
        <p><strong>{$t('settings.version')}:</strong> stable&work_2_[v61]</p>
        <p><strong>{$t('settings.date')}:</strong> 14.10.2026</p>
        <p><strong>{$t('settings.author')}:</strong> люченко ..</p>
        <p><strong>{$t('settings.organization')}:</strong> мск, мТ, а-261,    «ахаон»</p>
        <p><strong>{$t('settings.license')}:</strong> MIT</p>

        <h3>🔗 {$t('settings.github_title')}</h3>
        <div class="github-block">
          <code class="github-url">{githubUrl}</code>
          <div class="github-buttons">
            <button on:click={copyGithubUrl}>
              {#if githubCopied}✅ {$t('settings.github_copied')}{:else}📋 {$t('settings.github_copy')}{/if}
            </button>
            <button on:click={openGithubUrl}>➡️ {$t('settings.github_open')}</button>
          </div>
        </div>

      {:else if tab === 'appearance'}
        <h3>{$t('settings.theme_title')}</h3>
        <div class="radio-group">
          <label class="radio"><input type="radio" bind:group={$theme} value="light" /> ☀️ {$t('settings.theme_light')}</label>
          <label class="radio"><input type="radio" bind:group={$theme} value="dark" /> 🌙 {$t('settings.theme_dark')}</label>
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
        <div class="data-path"><code>{dataDir}</code></div>
        <div class="data-buttons">
          <button on:click={pickFolder} disabled={changing}>📁 {$t('settings.data_change')}</button>
          <button on:click={openInExplorer}>🔍 {$t('settings.data_open')}</button>
          <button on:click={resetToDefault} disabled={changing}>↩️ {$t('settings.data_reset')}</button>
        </div>
        {#if needsRestart}
          <p class="warning">⚠️ {$t('settings.data_restart')}</p>
        {/if}

      {:else if tab === 'reminders'}
        <h3>📌 {$t('settings.tray_title')}</h3>
        <p class="hint">{$t('settings.tray_hint')}</p>
        <label class="check-row">
          <input type="checkbox" checked={trayEnabled} on:change={toggleTray} />
          <span>{$t('settings.tray_enabled')}</span>
        </label>
        {#if trayChanged}
          <p class="warning">⚠️ {$t('settings.data_restart')}</p>
        {/if}

        <hr />

        <h3>🔔 {$t('settings.reminders_title')}</h3>
        <p class="hint">{$t('settings.reminders_hint')}</p>

        {#if reminders.length === 0}
          <p class="empty">{$t('settings.reminder_empty')}</p>
        {/if}

        <div class="reminders">
          {#each reminders as r (r.id)}
            <div class="reminder" class:disabled={!r.enabled}>
              <label class="reminder-toggle">
                <input
                  type="checkbox"
                  checked={r.enabled}
                  on:change={() => updateReminder(r.id, { enabled: !r.enabled })}
                />
              </label>

              <input
                class="reminder-time"
                type="time"
                value={r.time}
                on:change={(e) => updateReminder(r.id, { time: e.currentTarget.value })}
              />

              <div class="reminder-days">
                {#each dayKeys as dk, i}
                  <button
                    class="day-btn"
                    class:active={r.days.includes(i + 1)}
                    on:click={() => toggleDay(r.id, i + 1)}
                  >{$t('settings.' + dk)}</button>
                {/each}
              </div>

              <button class="reminder-delete" on:click={() => deleteReminder(r.id)} title={$t('settings.reminder_delete')}>🗑️</button>
            </div>
          {/each}
        </div>

        <button class="add-reminder" on:click={addReminder}>➕ {$t('settings.reminder_add')}</button>
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1200; }
  .dialog { background: var(--bg-card); color: var(--text); padding: 24px; border-radius: 8px; max-width: 700px; width: 90%; height: 600px; min-height: 400px; max-height: 85vh; display: flex; flex-direction: column; }
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  h2 { margin: 0; font-size: 1.2rem; }
  h3 { margin: 0 0 12px; font-size: 1rem; }
  p { margin: 4px 0; font-size: 0.9rem; color: var(--text-secondary); }
  hr { border: none; border-top: 1px solid var(--border); margin: 16px 0; }
  .hint { font-size: 0.85rem; color: var(--text-muted); margin-bottom: 12px; }
  .tabs { display: flex; gap: 6px; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 8px; flex-wrap: wrap; }
  .tabs button { background: none; border: none; padding: 8px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; color: var(--text-secondary); }
  .tabs button.active { background: var(--accent); color: white; }
  .content { overflow-y: auto; flex: 1; padding-right: 4px; }
  .radio-group { display: flex; flex-direction: column; gap: 8px; }
  .radio { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px; border-radius: 4px; font-size: 0.95rem; }
  .radio:hover { background: var(--bg-hover); }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--text); }

  .data-path { background: var(--bg-muted); border-radius: 4px; padding: 12px; margin-bottom: 12px; word-break: break-all; }
  .data-path code { font-family: monospace; font-size: 0.85rem; color: var(--text); }
  .data-buttons { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 12px; }
  .data-buttons button { background: var(--accent); color: white; border: none; padding: 8px 14px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .data-buttons button:disabled { opacity: 0.6; cursor: not-allowed; }
  .warning { color: var(--warning); font-size: 0.85rem; padding: 8px; background: var(--bg-muted); border-radius: 4px; }

  .check-row { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px 0; font-size: 0.95rem; }

  .empty { color: var(--text-muted); text-align: center; padding: 16px; font-style: italic; }
  .reminders { display: flex; flex-direction: column; gap: 8px; margin-bottom: 12px; }
  .reminder {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-card);
    flex-wrap: wrap;
  }
  .reminder.disabled { opacity: 0.5; }
  .reminder-toggle { display: flex; align-items: center; }
  .reminder-toggle input { width: 18px; height: 18px; cursor: pointer; }
  .reminder-time {
    padding: 6px 8px;
    border: 1px solid var(--border-input);
    border-radius: 4px;
    background: var(--bg-input);
    color: var(--text);
    font-family: monospace;
    font-size: 0.9rem;
  }
  .reminder-days { display: flex; gap: 4px; flex: 1; flex-wrap: wrap; }
  .day-btn {
    background: var(--bg-muted);
    color: var(--text-muted);
    border: none;
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.75rem;
    min-width: 34px;
    text-align: center;
  }
  .day-btn.active { background: var(--accent); color: white; }
  .reminder-delete {
    background: var(--danger-light);
    color: var(--danger);
    border: none;
    padding: 6px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .add-reminder {
    background: var(--accent);
    color: white;
    border: none;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    width: 100%;
  }

  .github-block {
    margin-top: 8px;
    margin-bottom: 16px;
  }
  .github-url {
    display: block;
    padding: 10px 12px;
    background: var(--bg-muted);
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8rem;
    word-break: break-all;
    color: var(--text);
    margin-bottom: 8px;
  }
  .github-buttons {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }
  .github-buttons button {
    background: var(--accent);
    color: white;
    border: none;
    padding: 8px 14px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .github-buttons button:hover {
    background: var(--accent-hover);
  }
</style>