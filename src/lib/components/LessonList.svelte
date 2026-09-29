<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { lessons, refreshLessons, removeLesson, markLessons } from '../stores/lessons';
  import { groups } from '../stores/groups';
  import { lessonsGroup, view, selectedLesson, confirmMessage, confirmCallback, selectedLessons } from '../stores/ui';
  import LessonForm from './LessonForm.svelte';

  let showForm = false;

  $: if ($lessonsGroup) refreshLessons($lessonsGroup);

  function openLesson(lesson: any) { $selectedLesson = lesson; }

  function toggleSelect(date: string) {
    const s = new Set($selectedLessons);
    if (s.has(date)) s.delete(date); else s.add(date);
    $selectedLessons = s;
  }

  function selectAll() {
    $selectedLessons = new Set($lessons.map(l => l.date));
  }

  function clearAll() { $selectedLessons = new Set(); }

  async function markSelected(marked: boolean) {
    if (!$lessonsGroup || $selectedLessons.size === 0) return;
    await markLessons($lessonsGroup, Array.from($selectedLessons), marked);
    $selectedLessons = new Set();
  }

  function askDelete(date: string, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = $t('lessons.confirm_delete');
    $confirmCallback = async () => {
      if ($lessonsGroup) await removeLesson($lessonsGroup, date);
      $confirmMessage = null;
      $confirmCallback = null;
    };
  }
</script>

<div class="lesson-list">
  <div class="header">
    <button class="back" on:click={() => { $view = 'groups'; $lessonsGroup = null; }}>← {$t('lessons.back')}</button>
    <select bind:value={$lessonsGroup}>
      <option value={null}>— выбери группу —</option>
      {#each $groups as g}
        <option value={g.name}>{g.name}</option>
      {/each}
    </select>
    {#if $lessonsGroup}
      <button class="add" on:click={() => showForm = true}>+ {$t('lessons.add')}</button>
    {/if}
  </div>

  {#if $lessonsGroup && $lessons.length > 0}
    <div class="bulk">
      <button on:click={selectAll}>Выбрать все</button>
      <button on:click={clearAll}>Снять все</button>
      <button on:click={() => markSelected(true)}>Отметить</button>
      <button on:click={() => markSelected(false)}>Снять отметку</button>
    </div>
  {/if}

  {#if !$lessonsGroup}
    <p class="empty">Выбери группу</p>
  {:else if $lessons.length === 0}
    <p class="empty">{$t('lessons.empty')}</p>
  {:else}
    {#each $lessons as lesson}
      <div class="lesson-item" role="button" tabindex="0" on:click={() => openLesson(lesson)} on:keydown={(e) => e.key === 'Enter' && openLesson(lesson)}>
        <input type="checkbox" checked={$selectedLessons.has(lesson.date)} on:click|stopPropagation={() => toggleSelect(lesson.date)} />
        <div class="info">
          <strong>{lesson.date}</strong>
          <span class="topic">{lesson.topic || '—'}</span>
          <span class="hours">{lesson.hours} ч.</span>
        </div>
        <span class="mark" class:marked={(lesson as any).marked}>{(lesson as any).marked ? '✓' : '○'}</span>
        <button class="delete" on:click={(e) => askDelete(lesson.date, e)}>×</button>
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
  .bulk button { background: #eee; border: none; padding: 6px 10px; border-radius: 4px; cursor: pointer; font-size: 0.8rem; }
  .empty { color: #999; text-align: center; padding: 32px; }
  .lesson-item { display: flex; align-items: center; gap: 12px; padding: 12px 16px; border: 1px solid #eee; border-radius: 6px; margin-bottom: 8px; cursor: pointer; }
  .lesson-item:hover { background: #f8f8f8; }
  .info { flex: 1; }
  .topic { color: #666; font-size: 0.85rem; margin-left: 12px; }
  .hours { color: #999; font-size: 0.85rem; margin-left: 12px; }
  .mark { font-size: 1.1rem; color: #ccc; }
  .mark.marked { color: #5cb85c; }
  .delete { background: none; border: none; color: #d9534f; font-size: 1.2rem; cursor: pointer; }
</style>