<!--
  ============================================================
  GroupForm.svelte — форма создания/редактирования группы
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { addGroup, updateGroup } from '../stores/groups';
  import { withMinLoading } from '../stores/ui';
  import type { Group } from '../utils/storage';

  export let onClose: () => void;
  export let editGroup: Group | null = null;

  let name = editGroup?.name || '';
  let defaultHours = editGroup?.default_hours ?? 1;
  let students: string[] = editGroup?.students?.length ? [...editGroup.students] : [''];
  let saving = false;

  function addStudent() { students = [...students, '']; }
  function removeStudent(i: number) { students = students.filter((_, idx) => idx !== i); }

  async function save() {
    if (!name.trim()) return;
    saving = true;
    const group: Group = {
      name: name.trim(),
      students: students.filter(s => s.trim()),
      default_hours: defaultHours
    };
    try {
      await withMinLoading(async () => {
        if (editGroup) await updateGroup(group);
        else await addGroup(group);
      });
    } finally {
      saving = false;
      onClose();
    }
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
  .dialog { background: var(--bg-card, white); color: var(--text, #333); padding: 24px; border-radius: 8px; max-width: 500px; width: 90%; max-height: 80vh; overflow-y: auto; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label, .label { display: block; margin-bottom: 12px; font-size: 0.9rem; color: #555; }
  input { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; }
  .student-row { display: flex; gap: 4px; margin-bottom: 4px; }
  .remove { background: #fdecea; color: #d9534f; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; }
  .add-student { background: none; border: 1px dashed #4a90d9; color: #4a90d9; padding: 8px; width: 100%; border-radius: 4px; cursor: pointer; margin-bottom: 16px; }
  .buttons { display: flex; gap: 8px; justify-content: flex-end; }
  button.cancel { background: #eee; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }
  button.save { background: #4a90d9; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; gap: 8px; min-width: 160px; justify-content: center; }
  button.save:disabled { opacity: 0.6; cursor: not-allowed; }
  .loader { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 0.6s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>