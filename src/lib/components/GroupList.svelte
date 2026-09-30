<!--
  ============================================================
  GroupList.svelte — список групп
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { groups, refreshGroups, removeGroup } from '../stores/groups';
  import { confirmMessage, confirmCallback } from '../stores/ui';
  import GroupForm from './GroupForm.svelte';

  let showForm = false;
  let editGroup: any = null;

  onMount(refreshGroups);

  function askDelete(name: string, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = $t('groups.confirm_delete');
    $confirmCallback = async () => {
      await removeGroup(name);
      $confirmMessage = null;
      $confirmCallback = null;
    };
  }

  function openEdit(group: any) { editGroup = group; showForm = true; }
  function openCreate() { editGroup = null; showForm = true; }
</script>

<div class="group-list">
  <div class="header">
    <h2>{$t('groups.title')}</h2>
    <button class="add" on:click={openCreate}>➕ {$t('groups.add')}</button>
  </div>

  {#if $groups.length === 0}
    <p class="empty">{$t('groups.empty')}</p>
  {:else}
    {#each $groups as group}
      <div class="group-item">
        <div class="info">
          <strong>{group.name}</strong>
          <span class="count">{group.students.length} {$t('groups.students_count')} · {group.default_hours} ч.</span>
        </div>
        <div class="actions">
          <button class="btn edit" on:click={() => openEdit(group)}>✏️ {$t('common.edit')}</button>
          <button class="btn delete" on:click={(e) => askDelete(group.name, e)}>🗑️ {$t('common.delete')}</button>
        </div>
      </div>
    {/each}
  {/if}
</div>

{#if showForm}
  <GroupForm editGroup={editGroup} onClose={() => { showForm = false; editGroup = null; }} />
{/if}

<style>
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  h2 { margin: 0; font-size: 1.1rem; color: var(--text); }
  .add { background: var(--accent); color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 0.9rem; }
  .add:hover { background: var(--accent-hover); }
  .empty { color: var(--text-muted); text-align: center; padding: 32px; }
  .group-item { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; border: 1px solid var(--border); background: var(--bg-card); border-radius: 6px; margin-bottom: 8px; }
  .info { flex: 1; }
  .count { color: var(--text-muted); font-size: 0.85rem; margin-left: 12px; }
  .actions { display: flex; gap: 8px; }
  .btn { border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .btn.edit { background: var(--accent-light); color: var(--accent); }
  .btn.delete { background: var(--danger-light); color: var(--danger); }
</style>