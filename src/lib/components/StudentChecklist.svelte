<!--
  ============================================================
  StudentChecklist.svelte — список учеников с отметками
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import type { Student } from '../utils/storage';

  export let students: Student[];
</script>

<div class="checklist">
  {#each students as student, i}
    <div class="student-row" class:absent={!student.present}>
      <label class="check-cell">
        <input type="checkbox" bind:checked={students[i].present} />
        <span class="name">{student.name}</span>
      </label>

      <input
        class="grade"
        placeholder={$t('lessons.grade_placeholder')}
        bind:value={students[i].grade}
      />

      {#if !student.present}
        <input
          class="reason"
          placeholder={$t('lessons.reason')}
          bind:value={students[i].reason}
        />
      {/if}
    </div>
  {/each}
</div>

<style>
  .student-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 0;
  }
  .student-row.absent .name { color: var(--text-muted); }
  .check-cell {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    cursor: pointer;
    min-width: 0;
  }
  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  input[type="checkbox"] { width: 18px; height: 18px; cursor: pointer; flex-shrink: 0; }
  .grade {
    width: 110px;
    padding: 4px 8px;
    border: 1px solid var(--border-input);
    border-radius: 4px;
    font-size: 0.85rem;
    background: var(--bg-input);
    color: var(--text);
    flex-shrink: 0;
  }
  .reason {
    flex: 1;
    min-width: 0;
    padding: 4px 8px;
    border: 1px solid var(--border-input);
    border-radius: 4px;
    font-size: 0.85rem;
    background: var(--bg-input);
    color: var(--text);
  }
</style>