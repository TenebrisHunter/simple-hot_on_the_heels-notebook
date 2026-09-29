# simple-hot_on_the_heels-notebook

Дневник занятий — быстрая фиксация того, что было на занятии: посещаемости, часов, тем и материалов с последующей возможностью перенести в стороннюю таблицу. Например, в Яндекс.Таблицы.

A lesson diary — quick recording of what happened in a lesson: attendance, hours, topics and materials, with the ability to transfer them to an external table. For example, to Yandex Sheets.

---

## Что это? / What is it?

**RU:** Десктопное приложение для педагогов. Позволяет после занятия быстро:
- отметить, кто был;
- записать часы, тему, материалы;
- скопировать данные в Яндекс.Таблицы как HTML-таблицу;
- хранить записи локально — без облака и регистрации.

Приложение создано, чтобы вы могли быстро зафиксировать, что прошло, а уже потом делать перенос всего в таблицу — в удобное для вас время. То есть вы по горячим следам записываете, а потом можете просматривать.

**EN:** A desktop app for teachers. Lets you quickly after a lesson:
- mark who was present;
- record hours, topic, materials;
- copy data to Yandex Sheets as an HTML table;
- store records locally — no cloud, no registration.

The app is made so you can quickly record what happened, and then transfer everything to a table later — at a time convenient for you. That is, you write it down while it is fresh, and can review it later.

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

## ⚠️ Персональные данные / Personal data

**RU:** Приложение хранит все данные **локально** на вашем компьютере, в папке `data/`. Данные **не передаются** в интернет, **не синхронизируются** с облаком и **не отправляются** разработчику.

Однако данные хранятся **в открытом виде** — в обычных текстовых файлах `.txt`. Любой, у кого есть доступ к вашему компьютеру и папке `data/`, сможет их прочитать.

**Внося в программу персональные данные других людей (ФИО учеников, причины пропусков и т.п.), вы делаете это осознанно и под свою ответственность.** Убедитесь, что у вас есть право хранить эти данные, и что ваш компьютер защищён паролем.

**EN:** The app stores all data **locally** on your computer, in the `data/` folder. Data is **not sent** to the internet, **not synced** to the cloud and **not sent** to the developer.

However, data is stored **in plain text** — in ordinary `.txt` files. Anyone with access to your computer and the `data/` folder can read it.

**By entering other people''s personal data into the program (student names, absence reasons, etc.), you do so knowingly and at your own responsibility.** Make sure you have the right to store this data, and that your computer is password-protected.

---

## Структура данных / Data structure

**RU:** Папка data/ рядом с приложением. Внутри:

**EN:** The data/ folder next to the app. Inside:

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

**RU:**
- Создание, редактирование, удаление.
- Часы по умолчанию — подставляются в новое занятие.
- Удаление — в корзину (30 дней).

**EN:**
- Create, edit, delete.
- Default hours — auto-filled in new lessons.
- Delete — moves to trash (30 days).

### Занятия / Lessons

**RU:**
- Создание, редактирование, удаление.
- Отметка посещаемости (галочки).
- Маркер «в журнале / не в журнале».
- Массовое выделение и массовое изменение маркера.
- Удаление одного / нескольких занятий.

**EN:**
- Create, edit, delete.
- Attendance checkboxes.
- Mark "in journal / not in journal".
- Bulk select and bulk mark.
- Delete one / multiple lessons.

### Корзина / Trash

**RU:**
- Занятия и группы хранятся 30 дней.
- Восстановление из корзины.
- Удаление навсегда.
- Автоочистка старых файлов.

**EN:**
- Lessons and groups stored 30 days.
- Restore from trash.
- Delete permanently.
- Auto-clean old files.

### Импорт / экспорт / Import / export

**RU:**
- Импорт txt из папки.
- HTML-копирование для Яндекс.Таблиц.
- Копирование отдельных полей.

**EN:**
- Import txt from folder.
- HTML copy for Yandex Sheets.
- Copy individual fields.

### UI / UX

**RU:**
- Спиннеры при сохранении, импорте, удалении.
- Подтверждение удаления.
- Иконки + текст на кнопках.

**EN:**
- Spinners on save, import, delete.
- Delete confirmation.
- Icons + text on buttons.

---

## Настройка / Customization

**RU:** Все параметры — в CSS-файлах компонентов. Меняются прямо в Svelte.

**EN:** All settings — in component CSS files. Edited directly in Svelte.

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
(сделал данную программу, потому что я ненавижу Яндекс.Таблицы)

**EN:**
Klyuchenko M.A.
Omsk, OmSTU, BIT-211
ANO TSO DO "Makhaon"
(Made this program because I hate Yandex Sheets)