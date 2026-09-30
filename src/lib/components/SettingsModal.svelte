<!--
  ============================================================
  SettingsModal.svelte — окно настроек
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
  Разделы:
    - О проекте (версия, автор, лицензия)
    - Тема (светлая / тёмная)
    - Язык (ru / en)
  ============================================================
-->
<script lang="ts">
  import { currentLocale } from '../i18n';
  import { theme, locale } from '../stores/settings';

  export let onClose: () => void;

  let tab: 'about' | 'appearance' | 'language' = 'about';

  function setLocale(l: 'ru' | 'en') {
    locale.set(l);
    currentLocale.set(l);
  }
</script>

<div class="overlay">
  <div class="dialog">
    <div class="header">
      <h2>⚙️ Настройки</h2>
      <button class="close" on:click={onClose}>×</button>
    </div>

    <div class="tabs">
      <button class:active={tab === 'about'} on:click={() => tab = 'about'}>ℹ️ О проекте</button>
      <button class:active={tab === 'appearance'} on:click={() => tab = 'appearance'}>🎨 Тема</button>
      <button class:active={tab === 'language'} on:click={() => tab = 'language'}>🌐 Язык</button>
    </div>

    <div class="content">
      {#if tab === 'about'}
        <h3>simple-hot_on_the_heels-notebook</h3>
        <p>Дневник занятий для педагога.</p>
        <p><strong>Версия:</strong> stable&work_1_[v35]</p>
        <p><strong>Дата:</strong> 29.09.2026</p>
        <p><strong>Автор:</strong> Ключенко М.А.</p>
        <p><strong>Организация:</strong> Омск, ОмГТУ, БИТ-211, АНО ЦО ДО «Махаон»</p>
        <p><strong>Лицензия:</strong> MIT</p>
        <hr />
        <p class="hint">⚠️ Данные хранятся локально и в открытом виде. Вносите персональные данные осознанно.</p>
      {:else if tab === 'appearance'}
        <h3>Тема оформления</h3>
        <div class="radio-group">
          <label class="radio">
            <input type="radio" bind:group={$theme} value="light" />
            ☀️ Светлая
          </label>
          <label class="radio">
            <input type="radio" bind:group={$theme} value="dark" />
            🌙 Тёмная
          </label>
        </div>
      {:else if tab === 'language'}
        <h3>Язык интерфейса</h3>
        <div class="radio-group">
          <label class="radio">
            <input type="radio" checked={$locale === 'ru'} on:change={() => setLocale('ru')} />
            🇷🇺 Русский
          </label>
          <label class="radio">
            <input type="radio" checked={$locale === 'en'} on:change={() => setLocale('en')} />
            🇬🇧 English
          </label>
          <label class="radio disabled">
            <input type="radio" disabled />
            🇨🇳 中文 (скоро)
          </label>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1200; }
  .dialog { background: var(--bg-card, white); color: var(--text, #333); padding: 24px; border-radius: 8px; max-width: 600px; width: 90%; max-height: 85vh; display: flex; flex-direction: column; }
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  h2 { margin: 0; font-size: 1.2rem; }
  h3 { margin: 0 0 12px; font-size: 1rem; }
  p { margin: 4px 0; font-size: 0.9rem; }
  hr { border: none; border-top: 1px solid #eee; margin: 12px 0; }
  .hint { color: #b8860b; font-size: 0.85rem; }
  .tabs { display: flex; gap: 6px; margin-bottom: 16px; border-bottom: 1px solid #eee; padding-bottom: 8px; }
  .tabs button { background: none; border: none; padding: 8px 12px; cursor: pointer; border-radius: 4px; font-size: 0.9rem; color: #555; }
  .tabs button.active { background: #4a90d9; color: white; }
  .content { overflow-y: auto; flex: 1; }
  .radio-group { display: flex; flex-direction: column; gap: 8px; }
  .radio { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 8px; border-radius: 4px; font-size: 0.95rem; }
  .radio:hover { background: #f5f5f5; }
  .radio.disabled { opacity: 0.5; cursor: not-allowed; }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
</style>