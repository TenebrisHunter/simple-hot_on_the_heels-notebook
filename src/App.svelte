<script lang="ts">
  import { t } from './lib/i18n';
  import GroupList from './lib/components/GroupList.svelte';
  import LessonList from './lib/components/LessonList.svelte';
  import LessonView from './lib/components/LessonView.svelte';
  import ImportView from './lib/components/ImportView.svelte';
  import TrashView from './lib/components/TrashView.svelte';
  import ConfirmDialog from './lib/components/ConfirmDialog.svelte';
  import { view, confirmMessage, confirmCallback } from './lib/stores/ui';
</script>

<main>
  <header>
    <h1>{$t('app.title')}</h1>
    <nav>
      <button class:active={$view === 'groups'} on:click={() => $view = 'groups'}>👥 {$t('nav.groups')}</button>
      <button class:active={$view === 'lessons'} on:click={() => $view = 'lessons'}>📚 {$t('nav.lessons')}</button>
      <button class:active={$view === 'import'} on:click={() => $view = 'import'}>📥 {$t('nav.import')}</button>
      <button class:active={$view === 'trash'} on:click={() => $view = 'trash'}>🗑️ {$t('nav.trash')}</button>
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
    Дневник занятий · Омск, ОмГТУ, БИТ-211 · Ключенко М.А. · 2026
  </footer>
</main>

{#if $confirmMessage && $confirmCallback}
  <ConfirmDialog message={$confirmMessage} onConfirm={$confirmCallback} onCancel={() => { $confirmMessage = null; $confirmCallback = null; }} />
{/if}

<LessonView />

<style>
  :global(body) { margin: 0; font-family: system-ui, -apple-system, sans-serif; background: #fafafa; }
  main { max-width: 900px; margin: 0 auto; padding: 16px; display: flex; flex-direction: column; min-height: 100vh; box-sizing: border-box; }
  header { display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; padding-bottom: 8px; margin-bottom: 16px; flex-wrap: wrap; gap: 8px; }
  h1 { font-size: 1.2rem; margin: 0; }
  nav { display: flex; gap: 8px; flex-wrap: wrap; }
  nav button { background: none; border: none; padding: 6px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; }
  nav button.active { background: #4a90d9; color: white; }
  section { flex: 1; }
  footer {
    margin-top: 24px;
    padding-top: 12px;
    border-top: 1px solid #eee;
    text-align: center;
    color: #999;
    font-size: 0.8rem;
  }
</style>