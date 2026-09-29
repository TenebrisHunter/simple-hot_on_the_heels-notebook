<script lang="ts">
  import { t } from '../i18n';
  import { addGroup } from '../stores/groups';

  export let onClose: () => void;

  let name = '';
  let students: string[] = [''];
  let saving = false;

  function addStudent() {
    students = [...students, ''];
  }

  function removeStudent(i: number) {
    students = students.filter((_, idx) => idx !== i);
  }

  async function save() {
    if (!name.trim()) return;
    saving = true;
    await addGroup({
      name: name.trim(),
      students: students.filter(s => s.trim())
    });
    saving = false;
    onClose();
  }
</script>

<div class="overlay">
  <div class="dialog">
    <h2>{$t('groups.add')}</h2>

    <label>
      {$t('groups.name')}
      <input bind:value={name} placeholder="Группа А" />
    </label>

    <label>{$t('groups.students')}</label>
    {#each students as _, i}
      <div class="student-row">
        <input bind:value={students[i]} placeholder="{$t('groups.student_name')}" />
        <button class="remove" on:click={() => removeStudent(i)}>×</button>
      </div>
    {/each}
    <button class="add-student" on:click={addStudent}>+ {$t('groups.add_student')}</button>

    <div class="buttons">
      <button class="cancel" on:click={onClose}>{$t('common.cancel')}</button>
      <button class="save" on:click={save} disabled={saving}>
        {saving ? $t('common.loading') : $t('common.save')}
      </button>
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
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
  }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label {
    display: block;
    margin-bottom: 12px;
    font-size: 0.9rem;
    color: #555;
  }
  input {
    width: 100%;
    padding: 8px;
    margin-top: 4px;
    border: 1px solid #ccc;
    border-radius: 4px;
    box-sizing: border-box;
  }
  .student-row {
    display: flex;
    gap: 4px;
    margin-bottom: 4px;
  }
  .remove {
    background: #d9534f;
    color: white;
    border: none;
    padding: 8px 12px;
    border-radius: 4px;
    cursor: pointer;
  }
  .add-student {
    background: none;
    border: 1px dashed #4a90d9;
    color: #4a90d9;
    padding: 8px;
    width: 100%;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 16px;
  }
  .buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
  button.cancel { background: #eee; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }
  button.save {
    background: #4a90d9;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
  }
</style>
