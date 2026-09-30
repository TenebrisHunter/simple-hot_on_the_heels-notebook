# simple-hot_on_the_heels-notebook

Дневник занятий — быстрая фиксация того, что было на занятии: посещаемости, часов, тем и материалов с последующей возможностью перенести в стороннюю таблицу. Например, в Яндекс.Таблицы.

A lesson diary — quick recording of what happened in a lesson: attendance, hours, topics and materials, with the ability to transfer them to an external table. For example, to Yandex Sheets.

课程日记 —— 快速记录课堂内容：出勤、学时、主题和材料，之后可转移到外部表格，例如 Yandex 表格。

**Версия / Version / 版本:** stable&work_1_[v50]
**Дата / Date / 日期:** 29.09.2026

---

## Что это? / What is it? / 这是什么？

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

**ZH:** 面向教师的桌面应用。下课后可快速：
- 勾选出勤情况；
- 记录学时、主题、材料；
- 将数据以 HTML 表格形式复制到 Yandex 表格；
- 本地存储记录 —— 无需云端、无需注册。

本应用旨在让您快速记录课堂内容，之后在方便的时候再统一转移到表格中。也就是说，趁记忆新鲜时先记下来，之后再回顾。

---

## Скриншот / Screenshot / 截图

(пока нет — добавим позже / none yet — will add later / 暂无，稍后补充)

---

## Стек / Stack / 技术栈

| Компонент / Component / 组件 | Технология / Technology / 技术 |
|---|---|
| UI | Svelte 5 + TypeScript |
| Ядро / Core / 核心 | Rust (Tauri) |
| Обёртка / Wrapper / 封装 | Tauri 2 |
| Хранение / Storage / 存储 | txt-файлы / txt files / txt 文件 (no DB) |
| Экспорт / Export / 导出 | HTML-таблица в буфер / HTML table to clipboard / HTML 表格到剪贴板 |
| Стили / Styles / 样式 | чистый CSS / plain CSS / 纯 CSS |
| Локализация / i18n / 本地化 | ru / en / zh |
| Тема / Theme / 主题 | светлая / тёмная / light / dark / 浅色 / 深色 |

---

## Установка / Installation / 安装

**RU:** Подробная инструкция — в файле INSTALL_RU.txt.

**EN:** Detailed instructions — see INSTALL_EN.txt.

**ZH:** 详细说明请参阅 INSTALL_RU.txt 或 INSTALL_EN.txt。 或 INSTALL_ZH.txt。

Кратко / Briefly / 简要 (для разработки / for development / 用于开发):

    pnpm install
    pnpm tauri dev

---

## ⚠️ Персональные данные / Personal data / 个人数据

**RU:** Приложение хранит все данные локально на вашем компьютере, в папке data/. Данные не передаются в интернет, не синхронизируются с облаком и не отправляются разработчику.

Однако данные хранятся в открытом виде — в обычных текстовых файлах .txt. Любой, у кого есть доступ к вашему компьютеру и папке data/, сможет их прочитать.

**Внося в программу персональные данные других людей (ФИО учеников, причины пропусков и т.п.), вы делаете это осознанно и под свою ответственность.** Убедитесь, что у вас есть право хранить эти данные, и что ваш компьютер защищён паролем.

**EN:** The app stores all data locally on your computer, in the data/ folder. Data is not sent to the internet, not synced to the cloud and not sent to the developer.

However, data is stored in plain text — in ordinary .txt files. Anyone with access to your computer and the data/ folder can read it.

**By entering other people''s personal data into the program (student names, absence reasons, etc.), you do so knowingly and at your own responsibility.** Make sure you have the right to store this data, and that your computer is password-protected.

**ZH:** 应用将所有数据本地存储在您电脑的 data/ 文件夹中。数据不会发送到互联网，不会同步到云端，也不会发送给开发者。

但数据以明文形式存储 —— 在普通的 .txt 文件中。任何能访问您电脑和 data/ 文件夹的人都可以读取。

**在程序中录入他人的个人数据（学生姓名、缺勤原因等）时，您是知情且自行承担责任的。** 请确保您有权存储这些数据，且您的电脑已设置密码保护。

---

## Настройки / Settings / 设置

**RU:** Меню настроек открывается кнопкой ⚙️ в правом верхнем углу. Разделы:

- **О проекте** — версия, автор, организация, лицензия.
- **Тема** — светлая или тёмная.
- **Язык** — русский, английский, китайский.

Настройки сохраняются автоматически.

**EN:** The settings menu opens with the ⚙️ button in the top right corner. Sections:

- **About** — version, author, organization, license.
- **Theme** — light or dark.
- **Language** — Russian, English, Chinese.

Settings are saved automatically.

**ZH:** 点击右上角的 ⚙️ 按钮打开设置菜单。包含：

- **关于** —— 版本、作者、组织、许可证。
- **主题** —— 浅色或深色。
- **语言** —— 俄语、英语、中文。

设置会自动保存。

---

## Структура данных / Data structure / 数据结构

**RU:** Папка data/ рядом с приложением. Внутри:

**EN:** The data/ folder next to the app. Inside:

**ZH:** 应用旁边的 data/ 文件夹。内部结构：

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

Формат имени файла занятия / Lesson file name format / 课程文件名格式:

    2026-09-22_14-30-45.txt
    │          │
    │          └─ время / time / 时间 (hh-mm-ss)
    └─ дата / date / 日期

**Многострочные поля** (тема, материалы) записываются через маркеры:

    Тема: <<<
    Первая строка
    Вторая строка
    >>>

---

## Возможности / Features / 功能

### Группы / Groups / 小组

**RU:**
- Создание, редактирование, удаление.
- Часы по умолчанию — подставляются в новое занятие.
- Удаление — в корзину (30 дней).

**EN:**
- Create, edit, delete.
- Default hours — auto-filled in new lessons.
- Delete — moves to trash (30 days).

**ZH:**
- 创建、编辑、删除。
- 默认学时 —— 自动填入新课程。
- 删除 —— 移入回收站（30 天）。

### Занятия / Lessons / 课程

**RU:**
- Создание, редактирование, удаление.
- Отметка посещаемости (галочки).
- Маркер «в журнале / не в журнале».
- Массовое выделение и массовое изменение маркера.
- Удаление одного / нескольких занятий.
- Просмотр занятия — таблица посещаемости.
- Многострочные тема и материалы.

**EN:**
- Create, edit, delete.
- Attendance checkboxes.
- Mark "in journal / not in journal".
- Bulk select and bulk mark.
- Delete one / multiple lessons.
- Lesson view — attendance table.
- Multi-line topic and materials.

**ZH:**
- 创建、编辑、删除。
- 出勤勾选。
- 标记「已录入 / 未录入」。
- 批量选择和批量标记。
- 删除单条 / 多条课程。
- 课程查看 —— 出勤表格。
- 多行主题和材料。

### Корзина / Trash / 回收站

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

**ZH:**
- 课程和小组保留 30 天。
- 从回收站恢复。
- 永久删除。
- 自动清理旧文件。

### Импорт / экспорт / Import / export / 导入 / 导出

**RU:**
- Импорт txt из папки.
- HTML-копирование для Яндекс.Таблиц.
- Копирование отдельных полей.

**EN:**
- Import txt from folder.
- HTML copy for Yandex Sheets.
- Copy individual fields.

**ZH:**
- 从文件夹导入 txt。
- 为 Yandex 表格复制 HTML。
- 复制单个字段。

### UI / UX / 界面

**RU:**
- Спиннеры при сохранении, импорте, удалении.
- Подтверждение удаления.
- Иконки + текст на кнопках.
- Тёмная и светлая тема.
- Три языка интерфейса.

**EN:**
- Spinners on save, import, delete.
- Delete confirmation.
- Icons + text on buttons.
- Dark and light theme.
- Three interface languages.

**ZH:**
- 保存、导入、删除时显示加载动画。
- 删除确认。
- 按钮带图标和文字。
- 深色和浅色主题。
- 三种界面语言。

---

## Настройка / Customization / 自定义

**RU:** Цвета меняются через CSS-переменные в App.svelte. Тема переключается автоматически.

**EN:** Colors are controlled via CSS variables in App.svelte. Theme switches automatically.

**ZH:** 颜色通过 App.svelte 中的 CSS 变量控制。主题自动切换。

| Переменная / Variable / 变量 | Что делает / What it does / 作用 | Светлая / Light / 浅色 | Тёмная / Dark / 深色 |
|---|---|---|---|
| --accent | Акцент / Accent / 强调色 | #4a90d9 | #5ba0e3 |
| --bg | Фон / Background / 背景 | #fafafa | #1a1a1a |
| --bg-card | Фон карточек / Card bg / 卡片背景 | #ffffff | #2a2a2a |
| --text | Текст / Text / 文本 | #333333 | #e0e0e0 |
| --border | Границы / Borders / 边框 | #eeeeee | #3a3a3a |
| --danger | Опасность / Danger / 危险 | #d9534f | #e57373 |
| --success | Успех / Success / 成功 | #2e7d32 | #66bb6a |

---

## Структура проекта / Project structure / 项目结构

    simple-hot_on_the_heels-notebook/
    ├── README.md
    ├── DESCRIPTION.txt
    ├── INSTALL_RU.txt
    ├── INSTALL_EN.txt
    ├── INSTALL_ZH.txt
    ├── LICENSE
    ├── TODO.txt
    ├── data/                       ← данные (gitignore)
    ├── src/                        ← Svelte
    │   ├── App.svelte
    │   ├── app.html
    │   ├── routes/
    │   └── lib/
    │       ├── components/
    │       ├── stores/
    │       ├── utils/
    │       └── i18n/
    │           └── locales/        ← ru.json, en.json, zh.json
    └── src-tauri/                  ← Rust
        ├── src/
        └── Cargo.toml

---

## Лицензия / License / 许可证

MIT — см. файл LICENSE / see LICENSE / 参见 LICENSE 文件。

---

## Авторы / Authors / 作者

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

**ZH:**
克柳琴科 М.А.
鄂木斯克，鄂木斯克国立技术大学，БИТ-211
АНО ЦО ДО «马哈翁»
（开发本程序是因为我讨厌 Yandex 表格）