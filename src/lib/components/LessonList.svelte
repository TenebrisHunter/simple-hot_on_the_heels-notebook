<script lang="ts">
  import { t } from '../i18n';
  import { lessons, refreshLessons, removeLesson, removeLessons, toggleMark } from '../stores/lessons';
  import { groups } from '../stores/groups';
  import { lessonsGroup, view, selectedLesson, confirmMessage, confirmCallback, selectedLessons } from '../stores/ui';
  import LessonForm from './LessonForm.svelte';
  import LoadingSpinner from './LoadingSpinner.svelte';

  let showForm = false;
  let deletingId: string | null = null;
  let deletingBulk = false;
  let togglingId: string | null = null;

  $: if ($lessonsGroup) refreshLessons($lessonsGroup);

  function openLesson(lesson: any) { $selectedLesson = lesson; }

  function toggleSelect(fileId: string) {
    const s = new Set($selectedLessons);
    if (s.has(fileId)) s.delete(fileId); else s.add(fileId);
    $selectedLessons = s;
  }

  function selectAll() { $selectedLessons = new Set($lessons.map(l => l.file_id || '')); }
  function clearAll() { $selectedLessons = new Set(); }

  async function onToggleMark(lesson: any, event: MouseEvent) {
    event.stopPropagation();
    if (!$lessonsGroup) return;
    togglingId = lesson.file_id;
    try {
      await toggleMark($lessonsGroup, lesson);
    } finally {
      togglingId = null;
    }
  }

  function askDeleteOne(lesson: any, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = `Удалить занятие от ${lesson.date} ${lesson.time}? Оно попадёт в корзину.`;
    $confirmCallback = async () => {
      $confirmMessage = null;
      $confirmCallback = null;
      if (!$lessonsGroup || !lesson.file_id) return;
      deletingId = lesson.file_id;
      try {
        await removeLesson($lessonsGroup, lesson.file_id);
      } finally {
        deletingId = null;
      }
    };
  }

  function askDeleteSelected() {
    if ($selectedLessons.size === 0) return;
    const count = $selectedLessons.size;
    $confirmMessage = `Удалить выбранные записи (${count})? Они попадут в корзину.`;
    $confirmCallback = async () => {
      $confirmMessage = null;
      $confirmCallback = null;
      if (!$lessonsGroup) return;
      deletingBulk = true;
      try {
        await removeLessons($lessonsGroup, Array.from($selectedLessons));
        $selectedLessons = new Set();
      } finally {
        deletingBulk = false;
      }
    };
  }
</script>

<div class="lesson-list">
  <div class="header">
    <button class="back" on:click={() => { $view = 'groups'; $lessonsGroup = null; }}>← {$t('lessons.back')}</button>
    <select bind:value={$lessonsGroup}>
      <option value={null}>— выбери группу —</option>
      {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
    </select>
    {#if $lessonsGroup}
      <button class="add" on:click={() => showForm = true}>➕ {$t('lessons.add')}</button>
    {/if}
  </div>

  {#if $lessonsGroup && $lessons.length > 0}
    <div class="bulk">
      <button on:click={selectAll}>✅ Выбрать все</button>
      <button on:click={clearAll}>🔄 Снять все</button>
      <button class="danger" on:click={askDeleteSelected} disabled={$selectedLessons.size === 0 || deletingBulk}>
        {#if deletingBulk}
          <LoadingSpinner active size={14} color="#d9534f" /> Удаление...
        {:else}
          🗑️ Удалить выбранные ({$selectedLessons.size})
        {/if}
      </button>
    </div>
  {/if}

  {#if !$lessonsGroup}
    <p class="empty">Выбери группу</p>
  {:else if $lessons.length === 0}
    <p class="empty">{$t('lessons.empty')}</p>
  {:else}
    {#each $lessons as lesson}
      <div class="lesson-item">
        <input type="checkbox" checked={$selectedLessons.has(lesson.file_id || '')} on:change={() => toggleSelect(lesson.file_id || '')} />
        <div class="info" role="button" tabindex="0" on:click={() => openLesson(lesson)} on:keydown={(e) => e.key === 'Enter' && openLesson(lesson)}>
          <strong>{lesson.date} {lesson.time}</strong>
          <span class="topic">{lesson.topic || '—'}</span>
          <span class="hours">{lesson.hours} ч.</span>
        </div>

        <button class="mark-btn" class:marked={lesson.marked} on:click={(e) => onToggleMark(lesson, e)} disabled={togglingId === lesson.file_id}>
          {#if togglingId === lesson.file_id}
            <LoadingSpinner active size={12} color="#4a90d9" />
          {:else}
            {lesson.marked ? '✓ В журнале' : '○ Не в журнале'}
          {/if}
        </button>

        <button class="btn delete" on:click={(e) => askDeleteOne(lesson, e)} disabled={deletingId === lesson.file_id} title="Удалить">
          {#if deletingId === lesson.file_id}
            <LoadingSpinner active size={14} color="#d9534f" />
          {:else}
            🗑️ Удалить
          {/if}
        </button>
      </div>
    {/each}
  {/if}
</div>

{#if showForm && $lessonsGroup}
  <LessonForm groupName={$lessonsGroup} onClose={() => showForm = false} />
{/if}

<style>
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; gap: 8px; }
  select { flex: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }
  .back, .add { background: #4a90d9; color: white; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .bulk { display: flex; gap: 6px; margin-bottom: 12px; flex-wrap: wrap; }
  .bulk button { background: #eee; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; display: inline-flex; align-items: center; gap: 6px; }
  .bulk button.danger { background: #fdecea; color: #d9534f; }
  .bulk button:disabled { opacity: 0.5; cursor: not-allowed; }
  .empty { color: #999; text-align: center; padding: 32px; }
  .lesson-item { display: flex; align-items: center; gap: 12px; padding: 12px 16px; border: 1px solid #eee; border-radius: 6px; margin-bottom: 8px; }
  .info { flex: 1; cursor: pointer; }
  .topic { color: #666; font-size: 0.85rem; margin-left: 12px; }
  .hours { color: #999; font-size: 0.85rem; margin-left: 12px; }
  .mark-btn { border: none; padding: 6px 10px; border-radius: 4px; cursor: pointer; font-size: 0.8rem; background: #f5f5f5; color: #999; display: inline-flex; align-items: center; gap: 6px; min-width: 110px; justify-content: center; }
  .mark-btn.marked { background: #e8f5e9; color: #2e7d32; }
  .mark-btn:disabled { opacity: 0.7; cursor: wait; }
  .btn { border: none; padding: 6px 10px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; display: inline-flex; align-items: center; gap: 6px; }
  .btn.delete { background: #fdecea; color: #d9534f; }
  .btn.delete:disabled { opacity: 0.7; cursor: wait; }
</style>