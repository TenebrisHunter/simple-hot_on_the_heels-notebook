// ============================================================
//  reminders.rs — фоновая задача напоминаний
//  Автор: Ключенко М.А. (Омск, ОмГТУ, ИБа-261)
//  Версия: stable&work_2_[v61]
// ============================================================
//  Логика:
//    - Каждые 60 секунд проверяем текущее время и день недели.
//    - Смотрим на список reminders в config.
//    - Если enabled + время совпало + день совпал + сегодня
//      ещё не показывали — показываем уведомление.
//    - Записываем last_notified[id] = сегодня.
// ============================================================

use chrono::Datelike;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// Запускает фоновый поток. Вызывается один раз при старте.
pub fn start_loop(app: AppHandle) {
    std::thread::spawn(move || {
        // Первая проверка — через минуту после старта
        std::thread::sleep(std::time::Duration::from_secs(60));
        loop {
            check_and_notify(&app);
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    });
}

fn check_and_notify(app: &AppHandle) {
    let now = chrono::Local::now();
    let time_str = now.format("%H:%M").to_string();
    let date_str = now.format("%Y-%m-%d").to_string();

    // День недели: 1=Пн ... 7=Вс
    let day_num: u32 = match now.weekday() {
        chrono::Weekday::Mon => 1,
        chrono::Weekday::Tue => 2,
        chrono::Weekday::Wed => 3,
        chrono::Weekday::Thu => 4,
        chrono::Weekday::Fri => 5,
        chrono::Weekday::Sat => 6,
        chrono::Weekday::Sun => 7,
    };

    let reminders = crate::config::get_reminders();
    let last_notified = crate::config::get_last_notified();

    for r in reminders {
        if !r.enabled { continue; }
        if r.time != time_str { continue; }
        if !r.days.contains(&day_num) { continue; }

        // Уже показывали сегодня?
        if let Some(d) = last_notified.get(&r.id) {
            if d == &date_str { continue; }
        }

        // Текст уведомления
        let text = r.text.clone().filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Не забудь записать прошедшие занятия".to_string());

        // Показываем уведомление
        let result = app.notification()
            .builder()
            .title("Дневник занятий")
            .body(&text)
            .show();

        match result {
            Ok(_) => {
                let _ = crate::config::set_last_notified(&r.id, &date_str);
                println!("[reminder] Уведомление показано: {} ({})", text, r.id);
            }
            Err(e) => {
                eprintln!("[reminder] Ошибка уведомления: {}", e);
            }
        }
    }
}