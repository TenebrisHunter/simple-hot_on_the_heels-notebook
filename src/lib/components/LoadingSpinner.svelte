<script lang="ts">
  export let active: boolean = false;
  export let size: number = 20;
  export let color: string = '#ffffff';
</script>

{#if active}
  <span
    class="spinner"
    style="width: {size}px; height: {size}px; border-color: {color}33; border-top-color: {color};"
  ></span>
{/if}

<style>
  .spinner {
    display: inline-block;
    border-width: 2px;
    border-style: solid;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    box-sizing: border-box;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>