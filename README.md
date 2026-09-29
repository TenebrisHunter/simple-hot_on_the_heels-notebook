# simple-hot_on_the_heels-notebook

Дневник занятий для тренера — быстрая фиксация посещаемости, часов, тем и материалов с последующим переносом в Яндекс.Таблицы.

A lesson diary for coaches — quick recording of attendance, hours, topics and materials, with easy transfer to Yandex Sheets.

---

## Что это? / What is it?

**RU:** Десктопное приложение для тренера. Позволяет после занятия быстро:
- отметить, кто был;
- записать часы, тему, материалы;
- скопировать данные в Яндекс.Таблицы как HTML-таблицу;
- хранить записи локально — без облака и регистрации.

Приложение создано, чтобы убрать рутину при переносе данных в журнал.

**EN:** A desktop app for coaches. Lets you quickly after a lesson:
- mark who was present;
- record hours, topic, materials;
- copy data to Yandex Sheets as an HTML table;
- store records locally — no cloud, no registration.

Made to remove the routine of transferring data into the journal.

---

## Скриншот / Screenshot

(пока нет — добавим позже / none yet — will add later)

---

## Стек / Stack

| Компонент / Component | Технология / Technology |
|---|---|
| UI | Svelte 5 + TypeScript |
| Ядро / Core | Rust (Tauri) |
| Обёртка / Wrapper | Tauri 2 |
| Хранение / Storage | txt-файлы / txt files (no DB) |
| Экспорт / Export | HTML-таблица в буфер / HTML table to clipboard |
| Стили / Styles | чистый CSS / plain CSS |
| Локализация / i18n | ru / en |

---

## Установка / Installation

**RU:** Подробная инструкция — в файле INSTALL_RU.txt.

**EN:** Detailed instructions — see INSTALL_EN.txt.

Кратко / Briefly (для разработки / for development):

    pnpm install
    pnpm tauri dev

---

## Структура данных / Data structure

Папка data/ рядом с приложением. Внутри:

    data/
    ├── groups/
    │   ├── Группа А/
    │   │   ├── group.txt                ← состав + часы по умолчанию
    │   │   ├── 2026-06-15_14-30-45.txt  ← занятие
    │   │   └── 2026-06-17_10-00-00.txt
    │   └── Группа Б/
    ├── trash/
    │   ├── lessons/                     ← удалённые занятия (30 дней)
    │   └── groups/                      ← удалённые группы (30 дней)
    └── imports/                         ← папка для импорта

Формат имени файла занятия / Lesson file name format:

    2026-09-22_14-30-45.txt
    │          │
    │          └─ время / time (hh-mm-ss)
    └─ дата / date

---

## Возможности / Features

### Группы / Groups
- Создание, редактирование, удаление / Create, edit, delete.
- Часы по умолчанию — подставляются в новое занятие / Default hours — auto-filled in new lessons.
- Удаление — в корзину (30 дней) / Delete — moves to trash (30 days).

### Занятия / Lessons
- Создание, редактирование, удаление / Create, edit, delete.
- Отметка посещаемости (галочки) / Attendance checkboxes.
- Маркер «в журнале / не в журнале» / Mark "in journal / not in journal".
- Массовое выделение и массовое изменение маркера / Bulk select and bulk mark.
- Удаление одного / нескольких занятий / Delete one / multiple lessons.

### Корзина / Trash
- Занятия и группы хранятся 30 дней / Lessons and groups stored 30 days.
- Восстановление из корзины / Restore from trash.
- Удаление навсегда / Delete permanently.
- Автоочистка старых файлов / Auto-clean old files.

### Импорт / экспорт / Import / export
- Импорт txt из папки / Import txt from folder.
- HTML-копирование для Яндекс.Таблиц / HTML copy for Yandex Sheets.
- Копирование отдельных полей / Copy individual fields.

### UI / UX
- Спиннеры при сохранении, импорте, удалении / Spinners on save, import, delete.
- Подтверждение удаления / Delete confirmation.
- Иконки + текст на кнопках / Icons + text on buttons.

---

## Настройка / Customization

Все параметры — в CSS-файлах компонентов. Меняются прямо в Svelte.

All settings — in component CSS files. Edited directly in Svelte.

| Параметр / Parameter | Что делает / What it does | По умолчанию / Default |
|---|---|---|
| font-size | Размер текста / Text size | 0.9rem |
| padding | Внутренние отступы / Inner padding | 8px |
| border-radius | Скругление / Border radius | 4px |
| background (active) | Цвет активной вкладки / Active tab color | #4a90d9 |

---

## Структура проекта / Project structure

    simple-hot_on_the_heels-notebook/
    ├── README.md
    ├── DESCRIPTION.txt
    ├── INSTALL_RU.txt
    ├── INSTALL_EN.txt
    ├── LICENSE
    ├── TODO.txt
    ├── data/                       ← данные (gitignore)
    ├── src/                        ← Svelte
    │   ├── App.svelte
    │   ├── routes/
    │   └── lib/
    │       ├── components/
    │       ├── stores/
    │       ├── utils/
    │       └── i18n/
    └── src-tauri/                  ← Rust
        ├── src/
        └── Cargo.toml

---

## Лицензия / License

MIT — см. файл LICENSE / see LICENSE.

---

## Авторы / Authors

**RU:**
Ключенко М.А.
Омск, ОмГТУ, БИТ-211
АНО ЦО ДО «Махаон»
(сделал данную программу, чтобы убрать рутину при переносе данных занятий в Яндекс.Таблицы)

**EN:**
Klyuchenko M.A.
Omsk, OmSTU, BIT-211
ANO TSO DO "Makhaon"
(Made this program to remove the routine of transferring lesson data to Yandex Sheets)