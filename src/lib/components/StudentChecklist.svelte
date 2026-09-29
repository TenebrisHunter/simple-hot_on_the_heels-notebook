<script lang="ts">
  import { t } from '../i18n';
  import type { Student } from '../utils/storage';

  export let students: Student[];
</script>

<div class="checklist">
  {#each students as student, i}
    <div class="student-row" class:absent={!student.present}>
      <label>
        <input type="checkbox" bind:checked={students[i].present} />
        <span>{student.name}</span>
      </label>
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
  .student-row.absent span { color: #999; }
  label {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    cursor: pointer;
  }
  input[type="checkbox"] { width: 18px; height: 18px; cursor: pointer; }
  .reason {
    flex: 1;
    padding: 4px 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 0.85rem;
  }
</style>
