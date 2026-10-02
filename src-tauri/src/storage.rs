// ============================================================
//  storage.rs — чтение/запись txt-файлов
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub name: String,
    pub present: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub students: Vec<String>,
    #[serde(default = "default_hours_value")]
    pub default_hours: f64,
}

fn default_hours_value() -> f64 { 1.0 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub date: String,
    pub time: String,
    pub hours: f64,
    pub topic: String,
    pub materials: String,
    pub students: Vec<Student>,
    #[serde(default)]
    pub marked: bool,
    #[serde(default)]
    pub file_id: String,
}

pub fn data_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("data");
    path.push("groups");
    path
}

pub fn trash_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("data");
    path.push("trash");
    path
}

pub fn load_groups() -> Result<Vec<Group>, String> {
    let dir = data_dir();
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; return Ok(vec![]); }
    let mut groups = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let group_file = path.join("group.txt");
            let mut students = vec![];
            let mut default_hours = 1.0;
            if group_file.exists() {
                let content = fs::read_to_string(&group_file).map_err(|e| e.to_string())?;
                for line in content.lines() {
                    if let Some(v) = line.strip_prefix("Часы: ") {
                        default_hours = v.trim().parse().unwrap_or(1.0);
                    } else if !line.trim().is_empty() {
                        students.push(line.to_string());
                    }
                }
            }
            groups.push(Group { name, students, default_hours });
        }
    }
    Ok(groups)
}

pub fn save_group(group: &Group) -> Result<(), String> {
    let dir = data_dir().join(&group.name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut content = String::new();
    content.push_str(&format!("Часы: {}\n", group.default_hours));
    for s in &group.students {
        content.push_str(&format!("{}\n", s));
    }
    fs::write(dir.join("group.txt"), content).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_group(name: &str) -> Result<(), String> {
    let src = data_dir().join(name);
    if !src.exists() { return Err("Группа не найдена".to_string()); }
    let trash = trash_dir().join("groups");
    fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
    let dst = trash.join(format!("{}_{}", name, timestamp()));
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_lessons(group_name: &str) -> Result<Vec<Lesson>, String> {
    let dir = data_dir().join(group_name);
    if !dir.exists() { return Ok(vec![]); }
    let mut lessons = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if name == "group.txt" { continue; }
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if let Ok(mut lesson) = parse_lesson(&content) {
                lesson.file_id = name.trim_end_matches(".txt").to_string();
                lessons.push(lesson);
            }
        }
    }
    lessons.sort_by(|a, b| format!("{}{}", a.date, a.time).cmp(&format!("{}{}", b.date, b.time)));
    Ok(lessons)
}

pub fn lesson_exists(group_name: &str, date: &str) -> Result<bool, String> {
    let dir = data_dir().join(group_name);
    if !dir.exists() { return Ok(false); }
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(date) && name.ends_with(".txt") { return Ok(true); }
    }
    Ok(false)
}

pub fn save_lesson(group_name: &str, lesson: &Lesson, overwrite: bool) -> Result<String, String> {
    let dir = data_dir().join(group_name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let base = format!("{}_{}", lesson.date, lesson.time.replace(":", "-"));
    let mut filename = format!("{}.txt", base);
    let mut path = dir.join(&filename);

    if !overwrite && path.exists() {
        let mut counter = 1;
        loop {
            filename = format!("{}_{}.txt", base, counter);
            path = dir.join(&filename);
            if !path.exists() { break; }
            counter += 1;
        }
    }

    fs::write(&path, format_lesson(lesson)).map_err(|e| e.to_string())?;
    Ok(filename.trim_end_matches(".txt").to_string())
}

pub fn delete_lesson(group_name: &str, file_id: &str) -> Result<(), String> {
    let src = data_dir().join(group_name).join(format!("{}.txt", file_id));
    if !src.exists() { return Err("Файл не найден".to_string()); }
    let trash = trash_dir().join("lessons").join(group_name);
    fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
    let dst = trash.join(format!("{}.txt", file_id));
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_lessons(group_name: &str, file_ids: Vec<String>) -> Result<usize, String> {
    let mut count = 0;
    for id in file_ids {
        if delete_lesson(group_name, &id).is_ok() { count += 1; }
    }
    Ok(count)
}

pub fn list_trash_lessons(group_name: &str) -> Result<Vec<String>, String> {
    let dir = trash_dir().join("lessons").join(group_name);
    if !dir.exists() { return Ok(vec![]); }
    let mut files = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".txt") { files.push(name.trim_end_matches(".txt").to_string()); }
    }
    Ok(files)
}

pub fn list_trash_groups() -> Result<Vec<String>, String> {
    let dir = trash_dir().join("groups");
    if !dir.exists() { return Ok(vec![]); }
    let mut groups = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if entry.path().is_dir() { groups.push(name); }
    }
    Ok(groups)
}

pub fn restore_trash_lesson(group_name: &str, file_id: &str) -> Result<(), String> {
    let src = trash_dir().join("lessons").join(group_name).join(format!("{}.txt", file_id));
    if !src.exists() { return Err("Файл не найден".to_string()); }
    let dst_dir = data_dir().join(group_name);
    fs::create_dir_all(&dst_dir).map_err(|e| e.to_string())?;
    let dst = dst_dir.join(format!("{}.txt", file_id));
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn restore_trash_group(trash_name: &str) -> Result<(), String> {
    let src = trash_dir().join("groups").join(trash_name);
    if !src.exists() { return Err("Группа не найдена".to_string()); }
    let real_name = trash_name.rsplitn(2, '_').nth(1).unwrap_or(trash_name);
    let dst = data_dir().join(real_name);
    if dst.exists() { return Err("Группа уже существует".to_string()); }
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_trash_lesson(group_name: &str, file_id: &str) -> Result<(), String> {
    let path = trash_dir().join("lessons").join(group_name).join(format!("{}.txt", file_id));
    if path.exists() { fs::remove_file(&path).map_err(|e| e.to_string())?; }
    Ok(())
}

pub fn delete_trash_group(trash_name: &str) -> Result<(), String> {
    let path = trash_dir().join("groups").join(trash_name);
    if path.exists() { fs::remove_dir_all(&path).map_err(|e| e.to_string())?; }
    Ok(())
}

pub fn clean_old_trash() -> Result<(), String> {
    let dir = trash_dir();
    if !dir.exists() { return Ok(()); }
    let now = std::time::SystemTime::now();
    let thirty_days = std::time::Duration::from_secs(30 * 24 * 60 * 60);
    fn walk(dir: &Path, now: std::time::SystemTime, max_age: std::time::Duration) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, now, max_age);
                } else if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if now.duration_since(modified).unwrap_or_default() > max_age {
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }
    walk(&dir, now, thirty_days);
    Ok(())
}

pub fn import_from_folder(folder_path: &str, group_name: &str) -> Result<usize, String> {
    let src = Path::new(folder_path);
    if !src.exists() { return Err("Папка не найдена".to_string()); }
    let dst = data_dir().join(group_name);
    fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
    let mut count = 0;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if name == "group.txt" { continue; }
            fs::copy(&path, dst.join(&name)).map_err(|e| e.to_string())?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn toggle_mark(group_name: &str, lesson: &Lesson) -> Result<(), String> {
    let mut updated = lesson.clone();
    updated.marked = !updated.marked;
    save_lesson(group_name, &updated, true)?;
    Ok(())
}

fn timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

/// Парсит занятие. Поддерживает многострочные поля через маркеры <<< >>>.
fn parse_lesson(content: &str) -> Result<Lesson, String> {
    let mut date = String::new();
    let mut time = String::new();
    let mut hours = 0.0;
    let mut topic = String::new();
    let mut materials = String::new();
    let mut marked = false;
    let mut students = vec![];

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        if let Some(v) = line.strip_prefix("Дата: ") { date = v.to_string(); i += 1; }
        else if let Some(v) = line.strip_prefix("Время: ") { time = v.to_string(); i += 1; }
        else if let Some(v) = line.strip_prefix("Часы: ") { hours = v.parse().unwrap_or(0.0); i += 1; }
        else if let Some(v) = line.strip_prefix("Отмечено: ") { marked = v.trim() == "да"; i += 1; }
        else if let Some(v) = line.strip_prefix("Тема: <<<") {
            i += 1;
            let mut buf = vec![];
            while i < lines.len() && lines[i].trim() != ">>>" {
                buf.push(lines[i]);
                i += 1;
            }
            topic = buf.join("\n");
            i += 1; // пропускаем >>>
        }
        else if let Some(v) = line.strip_prefix("Тема: ") { topic = v.to_string(); i += 1; }
        else if let Some(v) = line.strip_prefix("Материалы: <<<") {
            i += 1;
            let mut buf = vec![];
            while i < lines.len() && lines[i].trim() != ">>>" {
                buf.push(lines[i]);
                i += 1;
            }
            materials = buf.join("\n");
            i += 1;
        }
        else if let Some(v) = line.strip_prefix("Материалы: ") { materials = v.to_string(); i += 1; }
        else if line.contains('\t') {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                students.push(Student {
                    name: parts[0].to_string(),
                    present: parts[1] == "да",
                    reason: parts.get(2).map(|s| s.to_string()),
                });
            }
            i += 1;
        }
        else { i += 1; }
    }

    Ok(Lesson { date, time, hours, topic, materials, students, marked, file_id: String::new() })
}

/// Форматирует занятие. Многострочные поля — через маркеры <<< >>>.
fn format_lesson(lesson: &Lesson) -> String {
    let mut out = String::new();
    out.push_str(&format!("Дата: {}\n", lesson.date));
    out.push_str(&format!("Время: {}\n", lesson.time));
    out.push_str(&format!("Часы: {}\n", lesson.hours));
    out.push_str(&format!("Отмечено: {}\n", if lesson.marked { "да" } else { "нет" }));

    // Тема — многострочно
    if lesson.topic.contains('\n') || lesson.topic.trim().is_empty() {
        out.push_str("Тема: <<<\n");
        out.push_str(&lesson.topic);
        out.push_str("\n>>>\n");
    } else {
        out.push_str(&format!("Тема: {}\n", lesson.topic));
    }

    // Материалы — многострочно
    if lesson.materials.contains('\n') || lesson.materials.trim().is_empty() {
        out.push_str("Материалы: <<<\n");
        out.push_str(&lesson.materials);
        out.push_str("\n>>>\n");
    } else {
        out.push_str(&format!("Материалы: {}\n", lesson.materials));
    }

    out.push('\n');
    for s in &lesson.students {
        out.push_str(&format!("{}\t{}", s.name, if s.present { "да" } else { "нет" }));
        if let Some(r) = &s.reason { out.push_str(&format!("\t{}", r)); }
        out.push('\n');
    }
    out
}
/// Переименовывает папку группы. Все занятия переезжают автоматически.
pub fn rename_group(old_name: &str, new_name: &str) -> Result<(), String> {
    if old_name == new_name { return Ok(()); }
    let src = data_dir().join(old_name);
    let dst = data_dir().join(new_name);
    if !src.exists() { return Err("Группа не найдена".to_string()); }
    if dst.exists() { return Err("Группа с таким именем уже существует".to_string()); }
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}
