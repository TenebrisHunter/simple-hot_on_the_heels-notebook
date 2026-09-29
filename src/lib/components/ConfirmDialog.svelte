<script lang="ts">
  import { t } from '../i18n';
  export let message: string;
  export let onConfirm: () => void;
  export let onCancel: () => void;
</script>

<div class="overlay">
  <div class="dialog">
    <p>{message}</p>
    <div class="buttons">
      <button class="cancel" on:click={onCancel}>{$t('common.cancel')}</button>
      <button class="confirm" on:click={onConfirm}>{$t('common.confirm')}</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .dialog {
    background: white;
    padding: 24px;
    border-radius: 8px;
    max-width: 400px;
    width: 90%;
  }
  p { margin: 0 0 16px; }
  .buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
  button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .cancel { background: #eee; }
  .confirm { background: #d9534f; color: white; }
</style>
