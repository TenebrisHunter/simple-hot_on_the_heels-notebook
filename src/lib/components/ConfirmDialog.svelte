<!--
  ============================================================
  ConfirmDialog.svelte — диалог подтверждения
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { escapeKey } from '../utils/escapeAction';
  export let message: string;
  export let onConfirm: () => void;
  export let onCancel: () => void;
</script>

<div class="overlay" use:escapeKey={onCancel}>
  <div class="dialog">
    <p>{message}</p>
    <div class="buttons">
      <button class="cancel" on:click={onCancel}>{$t('common.cancel')}</button>
      <button class="confirm" on:click={onConfirm}>{$t('common.confirm')}</button>
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1100; }
  .dialog { background: var(--bg-card, white); color: var(--text, #333); padding: 24px; border-radius: 8px; max-width: 420px; width: 90%; }
  p { margin: 0 0 16px; font-size: 0.95rem; }
  .buttons { display: flex; gap: 8px; justify-content: flex-end; }
  button { padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; }
  .cancel { background: #eee; }
  .confirm { background: #4a90d9; color: white; }
</style>