<script lang="ts">
  export let active: boolean = false;
</script>

{#if active}
  <div class="loading-bar">
    <div class="block"></div>
    <div class="block"></div>
    <div class="block"></div>
    <div class="block"></div>
    <div class="block"></div>
  </div>
{/if}

<style>
  .loading-bar {
    display: flex;
    gap: 4px;
    justify-content: center;
    padding: 8px;
  }
  .block {
    width: 12px;
    height: 12px;
    background: #4a90d9;
    animation: pulse 1s infinite;
  }
  .block:nth-child(2) { animation-delay: 0.1s; }
  .block:nth-child(3) { animation-delay: 0.2s; }
  .block:nth-child(4) { animation-delay: 0.3s; }
  .block:nth-child(5) { animation-delay: 0.4s; }

  @keyframes pulse {
    0%, 100% { opacity: 0.3; transform: scale(0.8); }
    50% { opacity: 1; transform: scale(1); }
  }
</style>
