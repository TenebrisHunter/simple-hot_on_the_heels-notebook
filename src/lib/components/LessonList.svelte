<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '../i18n';
  import { lessons, refreshLessons, removeLesson } from '../stores/lessons';
  import { selectedGroup, view, selectedLesson, confirmMessage, confirmCallback } from '../stores/ui';
  import LessonForm from './LessonForm.svelte';

  let showForm = false;

  $: if ($selectedGroup) refreshLessons($selectedGroup);

  function openLesson(lesson: any) {
    $selectedLesson = lesson;
  }

  function askDelete(date: string, event: MouseEvent) {
    event.stopPropagation();
    $confirmMessage = $t('lessons.confirm_delete');
    $confirmCallback = async () => {
      if ($selectedGroup) await removeLesson($selectedGroup, date);
      $confirmMessage = null;
      $confirmCallback = null;
    };
  }
</script>

<div class="lesson-list">
  <div class="header">
    <button class="back" on:click={() => { $view = 'groups'; $selectedGroup = null; }}>
      ← {$t('lessons.back')}
    </button>
    <h2>{$selectedGroup}</h2>
    <button class="add" on:click={() => showForm = true}>+ {$t('lessons.add')}</button>
  </div>

  {#if $lessons.length === 0}
    <p class="empty">{$t('lessons.empty')}</p>
  {:else}
    {#each $lessons as lesson}
      <div class="lesson-item" on:click={() => openLesson(lesson)}>
        <div>
          <strong>{lesson.date}</strong>
          <span class="topic">{lesson.topic || '—'}</span>
          <span class="hours">{lesson.hours} ч.</span>
        </div>
        <button class="delete" on:click={(e) => askDelete(lesson.date, e)}>×</button>
      </div>
    {/each}
  {/if}
</div>

{#if showForm && $selectedGroup}
  <LessonForm groupName={$selectedGroup} onClose={() => showForm = false} />
{/if}

<style>
  .header {
    display: flex; justify-content: space-between; align-items: center;
    margin-bottom: 16px; gap: 8px;
  }
  h2 { margin: 0; font-size: 1rem; flex: 1; text-align: center; }
  .back, .add {
    background: #4a90d9; color: white; border: none;
    padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem;
  }
  .empty { color: #999; text-align: center; padding: 32px; }
  .lesson-item {
    display: flex; justify-content: space-between; align-items: center;
    padding: 12px 16px; border: 1px solid #eee; border-radius: 6px;
    margin-bottom: 8px; cursor: pointer;
  }
  .lesson-item:hover { background: #f8f8f8; }
  .topic { color: #666; font-size: 0.85rem; margin-left: 12px; }
  .hours { color: #999; font-size: 0.85rem; margin-left: 12px; }
  .delete { background: none; border: none; color: #d9534f; font-size: 1.2rem; cursor: pointer; }
</style>
