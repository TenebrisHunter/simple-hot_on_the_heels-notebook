<!--
  ============================================================
  StudentChecklist.svelte — список учеников с отметками
  втор: люченко .. (мск, мТ, Т-211)
  ============================================================
  аскладка строки ученика:
    [✓] мя  [причина, если нет]  [отметка]
  Шапка:  | ТТ
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import type { Student } from '../utils/storage';

  export let students: Student[];
</script>

<div class="checklist">
  <div class="header-row">
    <span class="header-name">{$t('lessons.student')}</span>
    <span class="header-grade">{$t('lessons.grade')}</span>
  </div>

  {#each students as student, i}
    <div class="student-row" class:absent={!student.present}>
      <label class="check-cell">
        <input type="checkbox" bind:checked={students[i].present} />
        <span class="name">{student.name}</span>
      </label>

      <span class="reason-cell">
        {#if !student.present}
          <input
            class="reason"
            placeholder={$t('lessons.reason')}
            bind:value={students[i].reason}
          />
        {/if}
      </span>

      <input
        class="grade"
        placeholder="4- | A+"
        bind:value={students[i].grade}
      />
    </div>
  {/each}
</div>

<style>
  .checklist { margin-bottom: 12px; }
  .header-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 0 4px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 4px;
    font-size: 0.75rem;
    text-transform: uppercase;
    color: var(--text-muted);
    letter-spacing: 0.5px;
  }
  .header-name { flex: 1; padding-left: 26px; }
  .header-grade { width: 110px; text-align: center; }

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

  .reason-cell {
    flex: 1;
    min-width: 0;
    display: flex;
  }
  .reason {
    width: 100%;
    padding: 4px 8px;
    border: 1px solid var(--border-input);
    border-radius: 4px;
    font-size: 0.85rem;
    background: var(--bg-input);
    color: var(--text);
  }

  .grade {
    width: 110px;
    padding: 4px 8px;
    border: 1px solid var(--border-input);
    border-radius: 4px;
    font-size: 0.85rem;
    background: var(--bg-input);
    color: var(--text);
    flex-shrink: 0;
    text-align: center;
    font-family: monospace;
  }
</style>