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
}

/// Возвращает путь к папке data/groups
pub fn data_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // выходим из src-tauri
    path.push("data");
    path.push("groups");
    path
}

/// Загрузить список групп
pub fn load_groups() -> Result<Vec<Group>, String> {
    let dir = data_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        return Ok(vec![]);
    }

    let mut groups = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let group_file = path.join("group.txt");
            let students = if group_file.exists() {
                let content = fs::read_to_string(&group_file).map_err(|e| e.to_string())?;
                content.lines().filter(|l| !l.trim().is_empty()).map(|l| l.to_string()).collect()
            } else {
                vec![]
            };
            groups.push(Group { name, students });
        }
    }
    Ok(groups)
}

/// Сохранить группу
pub fn save_group(group: &Group) -> Result<(), String> {
    let dir = data_dir().join(&group.name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let group_file = dir.join("group.txt");
    let content = group.students.join("\n");
    fs::write(&group_file, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Удалить группу
pub fn delete_group(name: &str) -> Result<(), String> {
    let dir = data_dir().join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Загрузить занятия группы
pub fn load_lessons(group_name: &str) -> Result<Vec<Lesson>, String> {
    let dir = data_dir().join(group_name);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut lessons = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if name == "group.txt" { continue; }
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if let Ok(lesson) = parse_lesson(&content) {
                lessons.push(lesson);
            }
        }
    }
    lessons.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(lessons)
}

/// Сохранить занятие
pub fn save_lesson(group_name: &str, lesson: &Lesson) -> Result<(), String> {
    let dir = data_dir().join(group_name);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let filename = format!("{}.txt", lesson.date);
    let path = dir.join(filename);
    let content = format_lesson(lesson);
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Удалить занятие
pub fn delete_lesson(group_name: &str, date: &str) -> Result<(), String> {
    let path = data_dir().join(group_name).join(format!("{}.txt", date));
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Импорт txt из папки
pub fn import_from_folder(folder_path: &str, group_name: &str) -> Result<usize, String> {
    let src = Path::new(folder_path);
    if !src.exists() {
        return Err("Папка не найдена".to_string());
    }
    let dst = data_dir().join(group_name);
    fs::create_dir_all(&dst).map_err(|e| e.to_string())?;

    let mut count = 0;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if name == "group.txt" { continue; }
            let dst_path = dst.join(&name);
            fs::copy(&path, &dst_path).map_err(|e| e.to_string())?;
            count += 1;
        }
    }
    Ok(count)
}

/// Парсит занятие из txt
fn parse_lesson(content: &str) -> Result<Lesson, String> {
    let mut date = String::new();
    let mut hours = 0.0;
    let mut topic = String::new();
    let mut materials = String::new();
    let mut students = vec![];

    for line in content.lines() {
        if let Some(v) = line.strip_prefix("Дата: ") { date = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Часы: ") { hours = v.parse().unwrap_or(0.0); }
        else if let Some(v) = line.strip_prefix("Тема: ") { topic = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Материалы: ") { materials = v.to_string(); }
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

    Ok(Lesson { date, hours, topic, materials, students })
}

/// Форматирует занятие в txt
fn format_lesson(lesson: &Lesson) -> String {
    let mut out = String::new();
    out.push_str(&format!("Дата: {}\n", lesson.date));
    out.push_str(&format!("Часы: {}\n", lesson.hours));
    out.push_str(&format!("Тема: {}\n", lesson.topic));
    out.push_str(&format!("Материалы: {}\n\n", lesson.materials));
    for s in &lesson.students {
        out.push_str(&format!("{}\t{}", s.name, if s.present { "да" } else { "нет" }));
        if let Some(r) = &s.reason {
            out.push_str(&format!("\t{}", r));
        }
        out.push('\n');
    }
    out
}