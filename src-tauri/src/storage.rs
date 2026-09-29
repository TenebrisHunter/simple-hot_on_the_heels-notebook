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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub date: String,
    pub hours: f64,
    pub topic: String,
    pub materials: String,
    pub students: Vec<Student>,
    #[serde(default)]
    pub marked: bool,
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
            let students = if group_file.exists() {
                fs::read_to_string(&group_file).map_err(|e| e.to_string())?
                    .lines().filter(|l| !l.trim().is_empty()).map(|l| l.to_string()).collect()
            } else { vec![] };
            groups.push(Group { name, students });
        }
    }
    Ok(groups)
}

pub fn save_group(group: &Group) -> Result<(), String> {
    let dir = data_dir().join(&group.name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("group.txt"), group.students.join("\n")).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_group(name: &str) -> Result<(), String> {
    let dir = data_dir().join(name);
    if dir.exists() { fs::remove_dir_all(&dir).map_err(|e| e.to_string())?; }
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
            if let Ok(lesson) = parse_lesson(&content) { lessons.push(lesson); }
        }
    }
    lessons.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(lessons)
}

pub fn save_lesson(group_name: &str, lesson: &Lesson) -> Result<(), String> {
    let dir = data_dir().join(group_name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.txt", lesson.date));
    fs::write(&path, format_lesson(lesson)).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_lesson(group_name: &str, date: &str) -> Result<(), String> {
    let src = data_dir().join(group_name).join(format!("{}.txt", date));
    if !src.exists() { return Err("Файл не найден".to_string()); }
    let trash = trash_dir().join(group_name);
    fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
    let dst = trash.join(format!("{}_{}.txt", date, timestamp()));
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

/// Список файлов в корзине для группы
pub fn list_trash(group_name: &str) -> Result<Vec<String>, String> {
    let dir = trash_dir().join(group_name);
    if !dir.exists() { return Ok(vec![]); }
    let mut files = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".txt") { files.push(name); }
    }
    Ok(files)
}

/// Восстановить из корзины
pub fn restore_from_trash(group_name: &str, filename: &str) -> Result<(), String> {
    let src = trash_dir().join(group_name).join(filename);
    if !src.exists() { return Err("Файл не найден".to_string()); }
    let date = filename.split('_').next().unwrap_or("").to_string();
    let dst = data_dir().join(group_name).join(format!("{}.txt", date));
    fs::rename(&src, &dst).map_err(|e| e.to_string())?;
    Ok(())
}

/// Удалить из корзины навсегда
pub fn delete_from_trash(group_name: &str, filename: &str) -> Result<(), String> {
    let path = trash_dir().join(group_name).join(filename);
    if path.exists() { fs::remove_file(&path).map_err(|e| e.to_string())?; }
    Ok(())
}

/// Очистить корзину от старых файлов (старше 30 дней)
pub fn clean_old_trash() -> Result<(), String> {
    let dir = trash_dir();
    if !dir.exists() { return Ok(()); }
    let now = std::time::SystemTime::now();
    let thirty_days = std::time::Duration::from_secs(30 * 24 * 60 * 60);
    for group_entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let group_entry = group_entry.map_err(|e| e.to_string())?;
        if !group_entry.path().is_dir() { continue; }
        for file_entry in fs::read_dir(group_entry.path()).map_err(|e| e.to_string())? {
            let file_entry = file_entry.map_err(|e| e.to_string())?;
            let metadata = file_entry.metadata().map_err(|e| e.to_string())?;
            if let Ok(modified) = metadata.modified() {
                if now.duration_since(modified).unwrap_or_default() > thirty_days {
                    let _ = fs::remove_file(file_entry.path());
                }
            }
        }
    }
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

fn timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

fn parse_lesson(content: &str) -> Result<Lesson, String> {
    let mut date = String::new();
    let mut hours = 0.0;
    let mut topic = String::new();
    let mut materials = String::new();
    let mut marked = false;
    let mut students = vec![];
    for line in content.lines() {
        if let Some(v) = line.strip_prefix("Дата: ") { date = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Часы: ") { hours = v.parse().unwrap_or(0.0); }
        else if let Some(v) = line.strip_prefix("Тема: ") { topic = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Материалы: ") { materials = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Отмечено: ") { marked = v == "да"; }
        else if line.contains('\t') {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                students.push(Student {
                    name: parts[0].to_string(),
                    present: parts[1] == "да",
                    reason: parts.get(2).map(|s| s.to_string()),
                });
            }
        }
    }
    Ok(Lesson { date, hours, topic, materials, students, marked })
}

fn format_lesson(lesson: &Lesson) -> String {
    let mut out = String::new();
    out.push_str(&format!("Дата: {}\n", lesson.date));
    out.push_str(&format!("Часы: {}\n", lesson.hours));
    out.push_str(&format!("Тема: {}\n", lesson.topic));
    out.push_str(&format!("Материалы: {}\n", lesson.materials));
    out.push_str(&format!("Отмечено: {}\n\n", if lesson.marked { "да" } else { "нет" }));
    for s in &lesson.students {
        out.push_str(&format!("{}\t{}", s.name, if s.present { "да" } else { "нет" }));
        if let Some(r) = &s.reason { out.push_str(&format!("\t{}", r)); }
        out.push('\n');
    }
    out
}