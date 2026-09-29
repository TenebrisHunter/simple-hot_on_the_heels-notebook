<script lang="ts">
  import { t } from '../i18n';
  import { selectedLesson, lessonsGroup } from '../stores/ui';
  import { copyAllAsHtml, copyStudentsAsHtml, copyTopic, copyMaterials, toClipboard, toClipboardHtml } from '../utils/clipboard';

  let copied = false;

  async function copyAll() {
    if (!$selectedLesson) return;
    const html = copyAllAsHtml($selectedLesson);
    const plain = $selectedLesson.students.map((s: any) => `${s.name}\t${s.present ? 'да' : 'нет'}`).join('\n');
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
      <p><strong>{$t('lessons.topic')}:</strong> {$selectedLesson.topic}</p>
      <p><strong>{$t('lessons.materials')}:</strong> {$selectedLesson.materials}</p>

      <h3>{$t('lessons.students')}</h3>
      <pre class="copyable">
        {#each $selectedLesson.students as s}
          {s.name}{'\t'}{s.present ? 'да' : 'нет'}{s.reason ? '\t' + s.reason : ''}{'\n'}
        {/each}
      </pre>

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
  .mark { font-size: 0.8rem; color: #999; padding: 4px 8px; border-radius: 4px; background: #f5f5f5; }
  .mark.marked { color: #2e7d32; background: #e8f5e9; }
  .copyable { background: #f8f8f8; padding: 12px; border-radius: 4px; font-family: monospace; font-size: 0.85rem; white-space: pre; user-select: text; overflow-x: auto; }
  .buttons { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 16px; }
  .buttons button { background: #4a90d9; color: white; border: none; padding: 8px 12px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
  .copied { color: #5cb85c; font-size: 0.85rem; margin-top: 8px; }
  .close { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
</style>