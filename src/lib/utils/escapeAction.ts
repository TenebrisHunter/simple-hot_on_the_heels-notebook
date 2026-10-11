// ============================================================
//  escapeAction.ts — Svelte action: закрытие по ESC
//  втор: люченко .. (мск, мТ, Т-211)
// ============================================================
//  спользование:
//    <div use:escapeKey={onClose}> ... </div>
//  ри нажатии ESC — вызывается callback.
// ============================================================

export function escapeKey(_node: HTMLElement, callback: () => void) {
  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      callback();
    }
  }
  window.addEventListener('keydown', onKey);
  return {
    destroy() { window.removeEventListener('keydown', onKey); }
  };
}