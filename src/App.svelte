<!--
  ============================================================
  App.svelte — главный компонент
  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
  Версия: stable&work_2_[v61]
  ============================================================
-->
<script lang="ts">
  import { t } from './lib/i18n';
  import GroupList from './lib/components/GroupList.svelte';
  import LessonList from './lib/components/LessonList.svelte';
  import LessonView from './lib/components/LessonView.svelte';
  import ImportView from './lib/components/ImportView.svelte';
  import TrashView from './lib/components/TrashView.svelte';
  import ConfirmDialog from './lib/components/ConfirmDialog.svelte';
  import SettingsModal from './lib/components/SettingsModal.svelte';
  import { view, confirmMessage, confirmCallback } from './lib/stores/ui';
  import { theme } from './lib/stores/settings';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  let showSettings = false;

  onMount(() => {
    document.documentElement.setAttribute('data-theme', $theme);
  });
</script>

<main>
  <header>
    <h1>{$t('app.title')}</h1>
    <nav>
      <button class:active={$view === 'groups'} on:click={() => $view = 'groups'}>👥 {$t('nav.groups')}</button>
      <button class:active={$view === 'lessons'} on:click={() => $view = 'lessons'}>📚 {$t('nav.lessons')}</button>
      <button class:active={$view === 'import'} on:click={() => $view = 'import'}>📥 {$t('nav.import')}</button>
      <button class:active={$view === 'trash'} on:click={() => $view = 'trash'}>🗑️ {$t('nav.trash')}</button>
      <button class="settings-btn" on:click={() => showSettings = true} title={$t('nav.settings')}>⚙️</button>
    </nav>
  </header>

  <section>
    {#if $view === 'groups'}<GroupList />
    {:else if $view === 'lessons'}<LessonList />
    {:else if $view === 'import'}<ImportView />
    {:else if $view === 'trash'}<TrashView />
    {/if}
  </section>

  <footer>
    <div class="authors">{$t('footer.authors')}</div>
    <div class="warning">{$t('footer.warning')}</div>
  </footer>
</main>

{#if $confirmMessage && $confirmCallback}
  <ConfirmDialog message={$confirmMessage} onConfirm={$confirmCallback} onCancel={() => { $confirmMessage = null; $confirmCallback = null; }} />
{/if}

{#if showSettings}
  <SettingsModal onClose={() => showSettings = false} />
{/if}

<LessonView />

<style>
  :global(:root) {
    /* Светлая тема */
    --bg: #fafafa;
    --bg-card: #ffffff;
    --bg-hover: #f5f7fa;
    --bg-input: #ffffff;
    --bg-muted: #f5f5f5;
    --text: #333333;
    --text-secondary: #555555;
    --text-muted: #999999;
    --border: #eeeeee;
    --border-input: #cccccc;
    --accent: #4a90d9;
    --accent-hover: #3a7bc0;
    --accent-light: #eef4fb;
    --danger: #d9534f;
    --danger-light: #fdecea;
    --success: #2e7d32;
    --success-light: #e8f5e9;
    --warning: #b8860b;
  }

  :global(:root[data-theme="dark"]) {
    /* Тёмная тема */
    --bg: #1a1a1a;
    --bg-card: #2a2a2a;
    --bg-hover: #333333;
    --bg-input: #333333;
    --bg-muted: #333333;
    --text: #e0e0e0;
    --text-secondary: #b0b0b0;
    --text-muted: #888888;
    --border: #3a3a3a;
    --border-input: #4a4a4a;
    --accent: #5ba0e3;
    --accent-hover: #4a90d9;
    --accent-light: #2a3a4a;
    --danger: #e57373;
    --danger-light: #3a2222;
    --success: #66bb6a;
    --success-light: #223322;
    --warning: #d4a017;
  }

  :global(body) {
    margin: 0;
    font-family: system-ui, -apple-system, sans-serif;
    background: var(--bg);
    color: var(--text);
    transition: background 0.2s, color 0.2s;
  }

  :global(input), :global(select), :global(textarea) {
    background: var(--bg-input);
    color: var(--text);
    border-color: var(--border-input);
  }

  main { max-width: 900px; margin: 0 auto; padding: 16px; display: flex; flex-direction: column; min-height: 100vh; box-sizing: border-box; }
  header { display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--border); padding-bottom: 8px; margin-bottom: 16px; flex-wrap: wrap; gap: 8px; }
  h1 { font-size: 1.2rem; margin: 0; }
  nav { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
  nav button { background: none; border: none; padding: 6px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; color: var(--text); }
  nav button.active { background: var(--accent); color: white; }
  nav button.settings-btn { font-size: 1.2rem; padding: 6px 10px; }
  section { flex: 1; }
  footer {
    margin-top: 24px;
    padding-top: 12px;
    border-top: 1px solid var(--border);
    text-align: center;
    color: var(--text-muted);
    font-size: 0.8rem;
  }
  footer .authors { margin-bottom: 4px; }
  footer .warning { color: var(--warning); font-size: 0.75rem; }
</style>