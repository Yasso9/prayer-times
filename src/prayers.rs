use crate::event::Event;
use crate::prayer::Prayer;
use crate::Config;
use chrono::Local;

// Returns the current prayer
pub fn current(config: &Config) -> Prayer {
    next(config).previous()
}

// Returns the next prayer
pub fn next(config: &Config) -> Prayer {
    let current_date = Local::now().date_naive();
    let current_time = Local::now().time();

    // List prayer in the current day in order
    for enum_prayer in Event::list() {
        let prayer = Prayer::new(enum_prayer, current_date, config);
        // Compare the prayer time with the current time
        if current_time <= prayer.date_time().time() {
            return prayer;
        }
    }

    // If no prayer in the current day found, return Time of fajr of tomorrow
    Prayer::new(Event::Isha, current_date, config).next()
}
// List all prayers of the day
pub fn list_prayers(config: &Config) -> [Prayer; 9] {
    let date = Local::now().date_naive();

    [
        Prayer::new(Event::Fajr, date, config),
        Prayer::new(Event::Sunrise, date, config),
        Prayer::new(Event::Dhuhr, date, config),
        Prayer::new(Event::Asr, date, config),
        Prayer::new(Event::Sunset, date, config),
        Prayer::new(Event::Maghrib, date, config),
        Prayer::new(Event::Isha, date, config),
        Prayer::new(Event::Midnight, date, config),
        Prayer::new(Event::Qiyam, date, config),
    ]
}
