<script lang="ts">
  import { t } from '../i18n';
  import { addLesson } from '../stores/lessons';
  import { groups } from '../stores/groups';
  import type { Lesson, Student } from '../utils/storage';
  import StudentChecklist from './StudentChecklist.svelte';

  export let groupName: string;
  export let onClose: () => void;

  let date = new Date().toISOString().slice(0, 10);
  let hours = 1;
  let topic = '';
  let materials = '';
  let saving = false;

  $: group = $groups.find(g => g.name === groupName);
  let editableStudents: Student[] = [];

  $: if (group && editableStudents.length === 0) {
    editableStudents = group.students.map((name): Student => ({ name, present: true, reason: '' }));
  }

  async function save() {
    saving = true;
    const lesson: Lesson = { date, hours, topic, materials, students: editableStudents, marked: false };
    await addLesson(groupName, lesson);
    saving = false;
    onClose();
  }
</script>

<div class="overlay">
  <div class="dialog">
    <h2>{$t('lessons.add')} — {groupName}</h2>

    <label>{$t('lessons.students')}</label>
    <StudentChecklist bind:students={editableStudents} />

    <label>{$t('lessons.date')}
      <input type="date" bind:value={date} />
    </label>

    <label>{$t('lessons.hours')}
      <input type="number" step="0.5" bind:value={hours} />
    </label>

    <label>{$t('lessons.topic')}<textarea bind:value={topic} rows="3"></textarea></label>
    <label>{$t('lessons.materials')}<textarea bind:value={materials} rows="2"></textarea></label>

    <div class="buttons">
      <button class="cancel" on:click={onClose}>{$t('common.cancel')}</button>
      <button class="save" on:click={save} disabled={saving}>
        {#if saving}<span class="loader"></span>{:else}{$t('common.save')}{/if}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .dialog { background: white; padding: 24px; border-radius: 8px; max-width: 600px; width: 90%; max-height: 85vh; overflow-y: auto; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  label { display: block; margin-bottom: 12px; font-size: 0.9rem; color: #555; }
  input, textarea { width: 100%; padding: 8px; margin-top: 4px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; font-family: inherit; }
  .buttons { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
  button.cancel { background: #eee; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }
  button.save { background: #4a90d9; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; gap: 8px; min-width: 120px; justify-content: center; }
  button.save:disabled { opacity: 0.6; cursor: not-allowed; }
  .loader { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 0.6s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>