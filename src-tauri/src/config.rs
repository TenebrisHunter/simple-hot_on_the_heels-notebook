// ============================================================
//  config.rs — конфигурация приложения
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
//  Версия: stable&work_2_[v61]
// ============================================================
//  Что хранит:
//    - data_dir        — путь к папке данных
//    - tray_enabled    — сворачивать в трей?
//    - reminders       — список напоминаний (время + дни)
//    - last_notified   — { id: дата } последнего уведомления
//  Где хранит:
//    - %APPDATA%\simple-hot_on_the_heels-notebook\config.json
// ============================================================

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Напоминание — время + дни недели + вкл/выкл
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reminder {
    pub id: String,
    pub time: String,          // "HH:MM"
    pub days: Vec<u32>,        // 1=Пн ... 7=Вс
    pub enabled: bool,
    #[serde(default)]
    pub text: Option<String>,  // опциональный текст
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub data_dir: Option<String>,
    #[serde(default)]
    pub tray_enabled: bool,
    #[serde(default)]
    pub reminders: Vec<Reminder>,
    #[serde(default)]
    pub last_notified: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            data_dir: None,
            tray_enabled: false,
            reminders: Vec::new(),
            last_notified: HashMap::new(),
        }
    }
}

pub fn config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("simple-hot_on_the_heels-notebook");
    path
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> Config {
    let path = config_file();
    if !path.exists() { return Config::default(); }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_config(config: &Config) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_file(), json).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Папка данных ---

pub fn get_data_dir() -> PathBuf {
    let config = load_config();
    if let Some(dir) = config.data_dir {
        let p = PathBuf::from(&dir);
        if p.exists() { return p; }
    }
    let mut fallback = config_dir();
    fallback.push("data");
    fallback
}

pub fn set_data_dir(path: &str) -> Result<(), String> {
    let p = PathBuf::from(path);
    if !p.exists() {
        return Err("Папка не существует".to_string());
    }
    let mut config = load_config();
    config.data_dir = Some(path.to_string());
    save_config(&config)
}

pub fn is_data_dir_configured() -> bool {
    let config = load_config();
    if let Some(dir) = config.data_dir {
        return PathBuf::from(&dir).exists();
    }
    false
}

// --- Трей ---

pub fn is_tray_enabled() -> bool {
    load_config().tray_enabled
}

pub fn set_tray_enabled(enabled: bool) -> Result<(), String> {
    let mut config = load_config();
    config.tray_enabled = enabled;
    save_config(&config)
}

// --- Напоминания ---

pub fn get_reminders() -> Vec<Reminder> {
    load_config().reminders
}

pub fn save_reminders(reminders: Vec<Reminder>) -> Result<(), String> {
    let mut config = load_config();
    config.reminders = reminders;
    save_config(&config)
}

pub fn get_last_notified() -> HashMap<String, String> {
    load_config().last_notified
}

pub fn set_last_notified(id: &str, date: &str) -> Result<(), String> {
    let mut config = load_config();
    config.last_notified.insert(id.to_string(), date.to_string());
    save_config(&config)
}