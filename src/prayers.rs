use crate::event::Event;
use crate::prayer::{get_prayer, Prayer};
use crate::Config;
use chrono::Local;

// Returns the current prayer
pub fn current(config: &Config) -> Prayer {
    next(config).previous(config)
}
// Returns the next prayer
pub fn next(config: &Config) -> Prayer {
    let current_date = Local::now().date_naive();
    let current_time = Local::now().time();

    // List prayer in the current day in order
    for enum_prayer in Event::list() {
        let prayer = get_prayer(enum_prayer, current_date, config);
        // Compare the prayer time with the current time
        if current_time <= prayer.date_time().time() {
            return prayer;
        }
    }

    // If no prayer in the current day found, return Time of fajr of tomorrow
    get_prayer(Event::Isha, current_date, config).next(config)
}
// List all prayers of the day
pub fn list_prayers(config: &Config) -> [Prayer; 6] {
    let date = Local::now().date_naive();

    [
        get_prayer(Event::Fajr, date, config),
        get_prayer(Event::Shourouk, date, config),
        get_prayer(Event::Dhuhr, date, config),
        get_prayer(Event::Asr, date, config),
        get_prayer(Event::Maghrib, date, config),
        get_prayer(Event::Isha, date, config),
    ]
}
