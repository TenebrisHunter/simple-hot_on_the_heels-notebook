<script lang="ts">
  export let active: boolean = false;
</script>

{#if active}
  <div class="loading-bar">
    <div class="fill"></div>
  </div>
{/if}

<style>
  .loading-bar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: rgba(74, 144, 217, 0.15);
    z-index: 2000;
    overflow: hidden;
  }
  .fill {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    width: 40%;
    background: linear-gradient(90deg, #4a90d9, #6fb1f5, #4a90d9);
    animation: slide 1s linear infinite;
    border-radius: 2px;
  }
  @keyframes slide {
    0%   { left: -40%; }
    100% { left: 100%; }
  }
</style>