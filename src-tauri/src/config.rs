// ============================================================
//  config.rs — конфигурация приложения
//  Автор: Ключенко М.А. (Омск, ОмГТУ, БИТ-211)
// ============================================================
//  Что хранит:
//    - data_dir — путь к папке данных (выбирает пользователь).
//  Где хранит:
//    - %APPDATA%\simple-hot_on_the_heels-notebook\config.json
//  Если config.json нет — используем fallback:
//    - %APPDATA%\simple-hot_on_the_heels-notebook\data
// ============================================================

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub data_dir: Option<String>,
}

/// Папка для конфига приложения
pub fn config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("simple-hot_on_the_heels-notebook");
    path
}

/// Путь к файлу config.json
pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}

/// Загрузить конфиг (если файла нет — дефолтный)
pub fn load_config() -> Config {
    let path = config_file();
    if !path.exists() { return Config::default(); }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

/// Сохранить конфиг
pub fn save_config(config: &Config) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_file(), json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Текущая папка данных.
/// 1. Если в конфиге указана и существует — используем её.
/// 2. Иначе — %APPDATA%\simple-hot_on_the_heels-notebook\data
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

/// Установить новую папку данных
pub fn set_data_dir(path: &str) -> Result<(), String> {
    let p = PathBuf::from(path);
    if !p.exists() {
        return Err("Папка не существует".to_string());
    }
    let mut config = load_config();
    config.data_dir = Some(path.to_string());
    save_config(&config)
}

/// Проверить: настроена ли папка данных вручную
pub fn is_data_dir_configured() -> bool {
    let config = load_config();
    if let Some(dir) = config.data_dir {
        return PathBuf::from(&dir).exists();
    }
    false
}