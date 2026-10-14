<!--
  ============================================================
  LessonList.svelte — список занятий группы
  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { lessons, refreshLessons, removeLesson, removeLessons, toggleMark } from '../stores/lessons';
  import { groups } from '../stores/groups';
  import { lessonsGroup, view, selectedLesson, confirmMessage, confirmCallback, selectedLessons, withMinLoading } from '../stores/ui';
  import LessonForm from './LessonForm.svelte';
  import LoadingSpinner from './LoadingSpinner.svelte';
  import { formatDate } from '../utils/formatDate';

  let showForm = false;
  let editingLesson: any = null;
  let deletingId: string | null = null;
  let deletingBulk = false;
  let togglingId: string | null = null;

  const KEY_SORT = 'sport-diary:lessons-sort';
  let sortAsc = (typeof localStorage !== 'undefined' ? localStorage.getItem(KEY_SORT) : 'asc') !== 'desc';

  function toggleSort() {
    sortAsc = !sortAsc;
    if (typeof localStorage !== 'undefined') localStorage.setItem(KEY_SORT, sortAsc ? 'asc' : 'desc');
  }

  $: sortedLessons = sortAsc ? $lessons : [...$lessons].reverse();
  $: if ($lessonsGroup) refreshLessons($lessonsGroup);

  function openView(lesson: any) { $selectedLesson = lesson; }
  function openEdit(lesson: any, event: MouseEvent) { event.stopPropagation(); editingLesson = lesson; showForm = true; }
  function openCreate() { editingLesson = null; showForm = true; }

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
    try { await withMinLoading(async () => { await toggleMark($lessonsGroup, lesson); }); }
    finally { togglingId = null; }
  }

  function askDeleteOne(lesson: any, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = $t('lessons.confirm_delete_one', { date: lesson.date, time: lesson.time });
    $confirmCallback = async () => {
      $confirmMessage = null; $confirmCallback = null;
      if (!$lessonsGroup || !lesson.file_id) return;
      deletingId = lesson.file_id;
      try { await withMinLoading(async () => { await removeLesson($lessonsGroup, lesson.file_id); }); }
      finally { deletingId = null; }
    };
  }

  function askDeleteSelected() {
    if ($selectedLessons.size === 0) return;
    const count = $selectedLessons.size;
    $confirmMessage = $t('lessons.confirm_delete_selected', { count });
    $confirmCallback = async () => {
      $confirmMessage = null; $confirmCallback = null;
      if (!$lessonsGroup) return;
      deletingBulk = true;
      try {
        await withMinLoading(async () => {
          await removeLessons($lessonsGroup, Array.from($selectedLessons));
          $selectedLessons = new Set();
        });
      } finally { deletingBulk = false; }
    };
  }
</script>

<div class="lesson-list">
  <div class="header">
    <button class="back" on:click={() => { $view = 'groups'; $lessonsGroup = null; }}>← {$t('lessons.back')}</button>
    <select bind:value={$lessonsGroup}>
      <option value={null}>{$t('trash.choose')}</option>
      {#each $groups as g}<option value={g.name}>{g.name}</option>{/each}
    </select>
    {#if $lessonsGroup}
      <button class="add" on:click={openCreate}>➕ {$t('lessons.add')}</button>
    {/if}
  </div>

  {#if $lessonsGroup && $lessons.length > 0}
    <div class="bulk">
      <button on:click={selectAll}>✅ {$t('lessons.select_all')}</button>
      <button on:click={clearAll}>🔄 {$t('lessons.clear_all')}</button>
      <button class="danger" on:click={askDeleteSelected} disabled={$selectedLessons.size === 0 || deletingBulk}>
        {#if deletingBulk}
          <LoadingSpinner active size={14} color="var(--danger)" /> {$t('lessons.deleting')}
        {:else}
          🗑️ {$t('lessons.delete_selected')} ({$selectedLessons.size})
        {/if}
      </button>
      <button class="sort-btn" on:click={toggleSort} title={sortAsc ? $t('lessons.sort_asc') : $t('lessons.sort_desc')}>
        {sortAsc ? '↓' : '↑'} {$t('lessons.sort_btn')}
      </button>
    </div>
  {/if}

  {#if !$lessonsGroup}
    <p class="empty">{$t('lessons.choose_group')}</p>
  {:else if $lessons.length === 0}
    <p class="empty">{$t('lessons.empty')}</p>
  {:else}
    {#each sortedLessons as lesson}
      <div class="lesson-item">
        <input type="checkbox" checked={$selectedLessons.has(lesson.file_id || '')} on:change={() => toggleSelect(lesson.file_id || '')} />
        <div class="info" role="button" tabindex="0" on:click={() => openView(lesson)} on:keydown={(e) => e.key === 'Enter' && openView(lesson)}>
          <strong>{formatDate(lesson.date)} {lesson.time}</strong>
          <span class="topic">{(lesson.topic || '—').split('\n')[0]}</span>
          <span class="hours">{lesson.hours} {$t('lessons.hours').toLowerCase()}</span>
        </div>

        <button class="mark-btn" class:marked={lesson.marked} on:click={(e) => onToggleMark(lesson, e)} disabled={togglingId === lesson.file_id}>
          {#if togglingId === lesson.file_id}
            <LoadingSpinner active size={12} color="var(--accent)" />
          {:else}
            {lesson.marked ? '✓ ' + $t('lessons.in_journal') : '○ ' + $t('lessons.not_in_journal')}
          {/if}
        </button>

        <button class="btn edit" on:click={(e) => openEdit(lesson, e)} title={$t('common.edit')}>✏️</button>
        <button class="btn delete" on:click={(e) => askDeleteOne(lesson, e)} disabled={deletingId === lesson.file_id} title={$t('common.delete')}>
          {#if deletingId === lesson.file_id}
            <LoadingSpinner active size={14} color="var(--danger)" />
          {:else}
            🗑️
          {/if}
        </button>
      </div>
    {/each}
  {/if}
</div>

{#if showForm && $lessonsGroup}
  <LessonForm groupName={$lessonsGroup} lessonToEdit={editingLesson} onClose={() => { showForm = false; editingLesson = null; }} />
{/if}

<style>
  .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; gap: 8px; flex-wrap: wrap; }
  select { flex: 1; min-width: 150px; padding: 8px; border: 1px solid var(--border-input); border-radius: 4px; background: var(--bg-input); color: var(--text); }
  .back, .add { background: var(--accent); color: white; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .back:hover, .add:hover { background: var(--accent-hover); }
  .bulk { display: flex; gap: 6px; margin-bottom: 12px; flex-wrap: wrap; align-items: center; }
  .bulk button { background: var(--bg-muted); color: var(--text); border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; display: inline-flex; align-items: center; gap: 6px; }
  .bulk button.danger { background: var(--danger-light); color: var(--danger); }
  .bulk button:disabled { opacity: 0.5; cursor: not-allowed; }
  .bulk .sort-btn { margin-left: auto; background: var(--bg-card); border: 1px solid var(--border); }
  .empty { color: var(--text-muted); text-align: center; padding: 32px; }
  .lesson-item { display: flex; align-items: center; gap: 8px; padding: 12px 16px; border: 1px solid var(--border); background: var(--bg-card); border-radius: 6px; margin-bottom: 8px; }
  .info { flex: 1; cursor: pointer; }
  .topic { color: var(--text-secondary); font-size: 0.85rem; margin-left: 12px; }
  .hours { color: var(--text-muted); font-size: 0.85rem; margin-left: 12px; }
  .mark-btn { border: none; padding: 6px 10px; border-radius: 4px; cursor: pointer; font-size: 0.8rem; background: var(--bg-muted); color: var(--text-muted); display: inline-flex; align-items: center; gap: 6px; min-width: 120px; justify-content: center; }
  .mark-btn.marked { background: var(--success-light); color: var(--success); }
  .mark-btn:disabled { opacity: 0.7; cursor: wait; }
  .btn { border: none; padding: 6px 10px; border-radius: 4px; cursor: pointer; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 6px; }
  .btn.edit { background: var(--accent-light); color: var(--accent); }
  .btn.delete { background: var(--danger-light); color: var(--danger); }
  .btn.delete:disabled { opacity: 0.7; cursor: wait; }
</style>