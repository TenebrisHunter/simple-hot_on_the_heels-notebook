<!--
  ============================================================
  SettingsModal.svelte — окно настроек
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
-->
<script lang="ts">
  import { t, currentLocale } from '../i18n';
  import { theme, locale } from '../stores/settings';
  import { escapeKey } from '../utils/escapeAction';

  export let onClose: () => void;

  let tab: 'about' | 'appearance' | 'language' = 'about';

  function setLocale(l: 'ru' | 'en' | 'zh') {
    locale.set(l);
    currentLocale.set(l);
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
    </div>

    <div class="content">
      {#if tab === 'about'}
        <h3>simple-hot_on_the_heels-notebook</h3>
        <p>{$t('app.title')}</p>
        <p><strong>{$t('settings.version')}:</strong> stable&work_2_[v61]</p>
        <p><strong>{$t('settings.date')}:</strong> 29.09.2026</p>
        <p><strong>{$t('settings.author')}:</strong> Ключенко М.А.</p>
        <p><strong>{$t('settings.organization')}:</strong> Омск, ОмГТУ, БИТ-211</p>
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
          <label class="radio">
            <input type="radio" checked={$locale === 'ru'} on:change={() => setLocale('ru')} />
            🇷🇺 {$t('settings.lang_ru')}
          </label>
          <label class="radio">
            <input type="radio" checked={$locale === 'en'} on:change={() => setLocale('en')} />
            🇬🇧 {$t('settings.lang_en')}
          </label>
          <label class="radio">
            <input type="radio" checked={$locale === 'zh'} on:change={() => setLocale('zh')} />
            🇨🇳 {$t('settings.lang_zh')}
          </label>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1200; }
  .dialog { background: var(--bg-card); color: var(--text); padding: 24px; border-radius: 8px; max-width: 600px; width: 90%; max-height: 85vh; display: flex; flex-direction: column; }
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  h2 { margin: 0; font-size: 1.2rem; color: var(--text); }
  h3 { margin: 0 0 12px; font-size: 1rem; color: var(--text); }
  p { margin: 4px 0; font-size: 0.9rem; color: var(--text-secondary); }
  .tabs { display: flex; gap: 6px; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 8px; }
  .tabs button { background: none; border: none; padding: 8px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; color: var(--text-secondary); }
  .tabs button.active { background: var(--accent); color: white; }
  .content { overflow-y: auto; flex: 1; }
  .radio-group { display: flex; flex-direction: column; gap: 8px; }
  .radio { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px; border-radius: 4px; font-size: 0.95rem; color: var(--text); }
  .radio:hover { background: var(--bg-hover); }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--text); }
</style>