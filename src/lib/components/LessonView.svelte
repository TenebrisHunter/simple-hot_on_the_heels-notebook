<!--
  ============================================================
  LessonView.svelte — просмотр занятия
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
  ============================================================
  Что делает:
    - Показывает данные занятия.
    - Многострочные поля — white-space: pre-wrap.
    - Посещаемость — ТАБЛИЦА (имя / был / причина).
    - Кнопки копирования в буфер для Яндекс.Таблиц.
  ============================================================
-->
<script lang="ts">
  import { t } from '../i18n';
  import { selectedLesson, lessonsGroup } from '../stores/ui';
  import { copyAllAsHtml, copyStudentsAsHtml, copyTopic, copyMaterials, toClipboard, toClipboardHtml } from '../utils/clipboard';

  let copied = false;

  async function copyAll() {
    if (!$selectedLesson) return;
    const html = copyAllAsHtml($selectedLesson);
    const plain = $selectedLesson.students.map((s: any) => `${s.name}\t${s.present ? 'да' : 'нет'}${s.reason ? '\t' + s.reason : ''}`).join('\n');
    try { await toClipboardHtml(html, plain); } catch { await toClipboard(plain); }
    copied = true; setTimeout(() => copied = false, 1500);
  }

  async function copyStudentsOnly() {
    if (!$selectedLesson) return;
    const html = copyStudentsAsHtml($selectedLesson);
    const plain = $selectedLesson.students.map((s: any) => `${s.name}\t${s.present ? 'да' : 'нет'}`).join('\n');
    try { await toClipboardHtml(html, plain); } catch { await toClipboard(plain); }
    copied = true; setTimeout(() => copied = false, 1500);
  }

  async function copyText(text: string) {
    await toClipboard(text);
    copied = true; setTimeout(() => copied = false, 1500);
  }

  function close() { $selectedLesson = null; }
</script>

{#if $selectedLesson}
  <div class="overlay">
    <div class="dialog">
      <div class="header">
        <h2>{$selectedLesson.date} {$selectedLesson.time} — {$lessonsGroup}</h2>
        <button class="close" on:click={close}>×</button>
      </div>

      <p><strong>Статус журнала:</strong>
        <span class="mark" class:marked={$selectedLesson.marked}>
          {$selectedLesson.marked ? '✓ В журнале' : '○ Не в журнале'}
        </span>
      </p>

      <p><strong>{$t('lessons.hours')}:</strong> {$selectedLesson.hours}</p>

      <div class="field">
        <strong>{$t('lessons.topic')}:</strong>
        <div class="multiline">{$selectedLesson.topic || '—'}</div>
      </div>

      <div class="field">
        <strong>{$t('lessons.materials')}:</strong>
        <div class="multiline">{$selectedLesson.materials || '—'}</div>
      </div>

      <h3>{$t('lessons.students')}</h3>
      <table class="attendance">
        <thead>
          <tr>
            <th>#</th>
            <th>Ученик</th>
            <th>Был</th>
            <th>Причина</th>
          </tr>
        </thead>
        <tbody>
          {#each $selectedLesson.students as s, i}
            <tr class:absent={!s.present}>
              <td class="num">{i + 1}</td>
              <td>{s.name}</td>
              <td class="center">
                {#if s.present}
                  <span class="yes">✓ да</span>
                {:else}
                  <span class="no">✗ нет</span>
                {/if}
              </td>
              <td class="reason">{s.reason || ''}</td>
            </tr>
          {/each}
        </tbody>
      </table>

      <div class="buttons">
        <button on:click={copyAll}>📋 {$t('lessons.copy_all')}</button>
        <button on:click={copyStudentsOnly}>👥 {$t('lessons.copy_students')}</button>
        <button on:click={() => copyText(copyTopic($selectedLesson))}>📝 {$t('lessons.copy_topic')}</button>
        <button on:click={() => copyText(copyMaterials($selectedLesson))}>🧰 {$t('lessons.copy_materials')}</button>
      </div>

      {#if copied}<p class="copied">{$t('lessons.copied')}</p>{/if}
    </div>
  </div>
{/if}

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .dialog { background: white; padding: 24px; border-radius: 8px; max-width: 700px; width: 90%; max-height: 85vh; overflow-y: auto; }
  .header { display: flex; justify-content: space-between; align-items: center; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  h3 { margin: 16px 0 8px; font-size: 0.95rem; }
  p { margin: 4px 0; font-size: 0.9rem; }
  .field { margin: 8px 0; font-size: 0.9rem; }
  .multiline {
    white-space: pre-wrap;
    word-wrap: break-word;
    margin-top: 4px;
    padding: 8px;
    background: #f9f9f9;
    border-radius: 4px;
    font-family: inherit;
    line-height: 1.4;
  }
  .mark { font-size: 0.8rem; color: #999; padding: 4px 8px; border-radius: 4px; background: #f5f5f5; }
  .mark.marked { color: #2e7d32; background: #e8f5e9; }

  /* Таблица посещаемости */
  .attendance {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.88rem;
    margin-top: 4px;
  }
  .attendance th {
    text-align: left;
    padding: 8px;
    background: #f5f7fa;
    border-bottom: 2px solid #e3e8ee;
    font-weight: 600;
    color: #555;
  }
  .attendance td {
    padding: 8px;
    border-bottom: 1px solid #eee;
  }
  .attendance tr.absent td { color: #999; }
  .attendance .num { width: 30px; color: #999; text-align: center; }
  .attendance .center { text-align: center; width: 90px; }
  .attendance .reason { color: #b8860b; font-size: 0.85rem; }
  .yes { color: #2e7d32; font-weight: 600; }
  .no { color: #d9534f; font-weight: 600; }

  .buttons { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 16px; }
  .buttons button { background: #4a90d9; color: white; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .copied { color: #5cb85c; font-size: 0.85rem; margin-top: 8px; }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
</style>