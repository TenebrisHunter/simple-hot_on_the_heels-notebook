<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { groups, refreshGroups, removeGroup } from '../stores/groups';
  import { selectedGroup, view, confirmMessage, confirmCallback } from '../stores/ui';
  import GroupForm from './GroupForm.svelte';

  let showForm = false;

  onMount(refreshGroups);

  function selectGroup(name: string) {
    $selectedGroup = name;
    $view = 'lessons';
  }

  function askDelete(name: string, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = $t('groups.confirm_delete');
    $confirmCallback = async () => {
      await removeGroup(name);
      $confirmMessage = null;
      $confirmCallback = null;
    };
  }
</script>

<div class="group-list">
  <div class="header">
    <h2>{$t('groups.title')}</h2>
    <button class="add" on:click={() => showForm = true}>+ {$t('groups.add')}</button>
  </div>

  {#if $groups.length === 0}
    <p class="empty">{$t('groups.empty')}</p>
  {:else}
    {#each $groups as group}
      <div class="group-item" on:click={() => selectGroup(group.name)}>
        <div>
          <strong>{group.name}</strong>
          <span class="count">{group.students.length} чел.</span>
        </div>
        <button class="delete" on:click={(e) => askDelete(group.name, e)}>×</button>
      </div>
    {/each}
  {/if}
</div>

{#if showForm}
  <GroupForm onClose={() => showForm = false} />
{/if}

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  h2 { margin: 0; font-size: 1.1rem; }
  .add {
    background: #4a90d9;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
  }
  .empty { color: #999; text-align: center; padding: 32px; }
  .group-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border: 1px solid #eee;
    border-radius: 6px;
    margin-bottom: 8px;
    cursor: pointer;
    transition: background 0.15s;
  }
  .group-item:hover { background: #f8f8f8; }
  .count { color: #999; font-size: 0.85rem; margin-left: 12px; }
  .delete {
    background: none;
    border: none;
    color: #d9534f;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 4px 8px;
  }
</style>
