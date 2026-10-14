// ============================================================
//  config.rs — конфигурация приложения
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
//  Версия: stable&work_2_[v61]
// ============================================================
//  Что хранит:
//    - data_dir              — путь к папке данных
//    - tray_enabled          — сворачивать в трей?
//    - notifications_enabled — показывать напоминания?
//    - notification_time     — время напоминания (HH:MM)
//    - last_notified_date    — дата последнего уведомления
//  Где хранит:
//    - %APPDATA%\simple-hot_on_the_heels-notebook\config.json
// ============================================================

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub data_dir: Option<String>,
    #[serde(default)]
    pub tray_enabled: bool,
    #[serde(default)]
    pub notifications_enabled: bool,
    #[serde(default = "default_notification_time")]
    pub notification_time: String,
    #[serde(default)]
    pub last_notified_date: Option<String>,
}

fn default_notification_time() -> String { "19:00".to_string() }

impl Default for Config {
    fn default() -> Self {
        Config {
            data_dir: None,
            tray_enabled: false,
            notifications_enabled: false,
            notification_time: default_notification_time(),
            last_notified_date: None,
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

pub fn is_tray_enabled() -> bool {
    load_config().tray_enabled
}

pub fn set_tray_enabled(enabled: bool) -> Result<(), String> {
    let mut config = load_config();
    config.tray_enabled = enabled;
    save_config(&config)
}

pub fn is_notifications_enabled() -> bool {
    load_config().notifications_enabled
}

pub fn set_notifications_enabled(enabled: bool) -> Result<(), String> {
    let mut config = load_config();
    config.notifications_enabled = enabled;
    save_config(&config)
}

pub fn get_notification_time() -> String {
    load_config().notification_time
}

pub fn set_notification_time(time: &str) -> Result<(), String> {
    // Проверяем формат HH:MM
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return Err("Неверный формат времени (нужно HH:MM)".to_string());
    }
    let h: u32 = parts[0].parse().map_err(|_| "Неверный час".to_string())?;
    let m: u32 = parts[1].parse().map_err(|_| "Неверные минуты".to_string())?;
    if h > 23 || m > 59 {
        return Err("Час 0-23, минуты 0-59".to_string());
    }
    let mut config = load_config();
    config.notification_time = time.to_string();
    save_config(&config)
}

pub fn set_last_notified_date(date: &str) -> Result<(), String> {
    let mut config = load_config();
    config.last_notified_date = Some(date.to_string());
    save_config(&config)
}