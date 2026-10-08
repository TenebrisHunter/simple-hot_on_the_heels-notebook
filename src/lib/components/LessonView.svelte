<!--
  ============================================================
  LessonView.svelte — просмотр занятия
  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
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
    const plain = $selectedLesson.students.map((s: any) => `${s.name}\t${s.present ? $t('common.yes').toLowerCase() : $t('common.no').toLowerCase()}${s.reason ? '\t' + s.reason : ''}${s.grade ? '\t' + s.grade : ''}`).join('\n');
    try { await toClipboardHtml(html, plain); } catch { await toClipboard(plain); }
    copied = true; setTimeout(() => copied = false, 1500);
  }

  async function copyStudentsOnly() {
    if (!$selectedLesson) return;
    const html = copyStudentsAsHtml($selectedLesson);
    const plain = $selectedLesson.students.map((s: any) => `${s.name}\t${s.present ? $t('common.yes').toLowerCase() : $t('common.no').toLowerCase()}`).join('\n');
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

      <p><strong>{$t('lessons.status')}:</strong>
        <span class="mark" class:marked={$selectedLesson.marked}>
          {$selectedLesson.marked ? '✓ ' + $t('lessons.in_journal') : '○ ' + $t('lessons.not_in_journal')}
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
            <th>{$t('lessons.number')}</th>
            <th>{$t('lessons.student')}</th>
            <th>{$t('lessons.present')}</th>
            <th>{$t('lessons.grade')}</th>
            <th>{$t('lessons.reason')}</th>
          </tr>
        </thead>
        <tbody>
          {#each $selectedLesson.students as s, i}
            <tr class:absent={!s.present}>
              <td class="num">{i + 1}</td>
              <td>{s.name}</td>
              <td class="center">
                {#if s.present}
                  <span class="yes">✓ {$t('common.yes')}</span>
                {:else}
                  <span class="no">✗ {$t('common.no')}</span>
                {/if}
              </td>
              <td class="grade-cell">{s.grade || ''}</td>
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
  .dialog { background: var(--bg-card); color: var(--text); padding: 24px; border-radius: 8px; max-width: 750px; width: 90%; max-height: 85vh; overflow-y: auto; }
  .header { display: flex; justify-content: space-between; align-items: center; }
  h2 { margin: 0 0 16px; font-size: 1.1rem; }
  h3 { margin: 16px 0 8px; font-size: 0.95rem; }
  p { margin: 4px 0; font-size: 0.9rem; }
  .field { margin: 8px 0; font-size: 0.9rem; }
  .multiline { white-space: pre-wrap; word-wrap: break-word; margin-top: 4px; padding: 8px; background: var(--bg-muted); border-radius: 4px; font-family: inherit; line-height: 1.4; }
  .mark { font-size: 0.8rem; color: var(--text-muted); padding: 4px 8px; border-radius: 4px; background: var(--bg-muted); }
  .mark.marked { color: var(--success); background: var(--success-light); }
  .attendance { width: 100%; border-collapse: collapse; font-size: 0.88rem; margin-top: 4px; }
  .attendance th { text-align: left; padding: 8px; background: var(--bg-hover); border-bottom: 2px solid var(--border); font-weight: 600; color: var(--text-secondary); }
  .attendance td { padding: 8px; border-bottom: 1px solid var(--border); }
  .attendance tr.absent td { color: var(--text-muted); }
  .attendance .num { width: 30px; color: var(--text-muted); text-align: center; }
  .attendance .center { text-align: center; width: 90px; }
  .attendance .grade-cell { width: 100px; font-family: monospace; }
  .attendance .reason { color: var(--warning); font-size: 0.85rem; }
  .yes { color: var(--success); font-weight: 600; }
  .no { color: var(--danger); font-weight: 600; }
  .buttons { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 16px; }
  .buttons button { background: var(--accent); color: white; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .copied { color: var(--success); font-size: 0.85rem; margin-top: 8px; }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--text); }
</style>