<!--
  ============================================================
  GroupForm.svelte — форма создания/редактирования группы
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
  При переименовании группы, если в корзине есть удалённые
  занятия этой группы — показываем предупреждение.
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { addGroup, updateGroup } from '../stores/groups';
  import { withMinLoading, confirmMessage, confirmCallback } from '../stores/ui';
  import { hasTrashForGroup } from '../utils/storage';
  import type { Group } from '../utils/storage';

  export let onClose: () => void;
  export let editGroup: Group | null = null;

  const oldName = editGroup?.name || '';
  let name = editGroup?.name || '';
  let defaultHours = editGroup?.default_hours ?? 1;
  let students: string[] = editGroup?.students?.length ? [...editGroup.students] : [''];
  let saving = false;

  function addStudent() { students = [...students, '']; }
  function removeStudent(i: number) { students = students.filter((_, idx) => idx !== i); }

  async function doSave() {
    saving = true;
    const group: Group = {
      name: name.trim(),
      students: students.filter(s => s.trim()),
      default_hours: defaultHours
    };
    try {
      await withMinLoading(async () => {
        if (editGroup) await updateGroup(oldName, group);
        else await addGroup(group);
      });
    } finally {
      saving = false;
      onClose();
    }
  }

  async function save() {
    if (!name.trim()) return;

    // Проверяем: если имя меняется и есть корзина — предупреждаем
    if (editGroup && oldName !== name.trim()) {
      const hasTrash = await hasTrashForGroup(oldName);
      if (hasTrash) {
        $confirmMessage = $t('groups.confirm_rename_trash', { old: oldName, new: name.trim() });
        $confirmCallback = () => {
          $confirmMessage = null;
          $confirmCallback = null;
          doSave();
        };
        return;
      }
    }

    await doSave();
  }
</script>

<div class="overlay">
  <div class="dialog">
    <h2>{editGroup ? $t('groups.edit') : $t('groups.add')}</h2>

    <label>
      {$t('groups.name')}
      <input bind:value={name} placeholder="Группа А" />
    </label>

    <label>
      {$t('groups.default_hours')}
      <input type="number" step="0.5" bind:value={defaultHours} min="0" />
    </label>

    <div class="label">{$t('groups.students')}</div>
    {#each students as _, i}
      <div class="student-row">
        <input bind:value={students[i]} placeholder={$t('groups.student_name')} />
        <button class="remove" on:click={() => removeStudent(i)}>🗑️</button>
      </div>
    {/each}
    <button class="add-student" on:click={addStudent}>➕ {$t('groups.add_student')}</button>

    <div class="buttons">
      <button class="cancel" on:click={onClose}>{$t('common.cancel')}</button>
      <button class="save" on:click={save} disabled={saving}>
        {#if saving}<span class="loader"></span> {$t('common.loading')}{:else}💾 {$t('common.save')}{/if}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .dialog { background: var(--bg-card); color: var(--text); padding: 24px; border-radius: 8px; max-width: 500px; width: 90%; max-height: 80vh; overflow-y: auto; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label, .label { display: block; margin-bottom: 12px; font-size: 0.9rem; color: var(--text-secondary); }
  input { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid var(--border-input); border-radius: 4px; box-sizing: border-box; background: var(--bg-input); color: var(--text); }
  .student-row { display: flex; gap: 4px; margin-bottom: 4px; }
  .remove { background: var(--danger-light); color: var(--danger); border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; }
  .add-student { background: none; border: 1px dashed var(--accent); color: var(--accent); padding: 8px; width: 100%; border-radius: 4px; cursor: pointer; margin-bottom: 16px; }
  .buttons { display: flex; gap: 8px; justify-content: flex-end; }
  button.cancel { background: var(--bg-muted); color: var(--text); border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }
  button.save { background: var(--accent); color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; gap: 8px; min-width: 160px; justify-content: center; }
  button.save:disabled { opacity: 0.6; cursor: not-allowed; }
  .loader { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 0.6s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>