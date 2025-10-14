use crate::event::Event;
use crate::prayer::Prayer;
use crate::Config;
use chrono::{Local, NaiveDate};

pub fn current(config: &Config) -> Prayer {
    next(config).previous()
}

pub fn next(config: &Config) -> Prayer {
    let current_date = Local::now().date_naive();
    let current_time = Local::now().time();

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

pub fn list_prayers_for_date(config: &Config, date: NaiveDate) -> [Prayer; 8] {
    [
        Prayer::new(Event::Fajr, date, config),
        Prayer::new(Event::Sunrise, date, config),
        Prayer::new(Event::Dhuhr, date, config),
        Prayer::new(Event::Asr, date, config),
        Prayer::new(Event::Sunset, date, config),
        Prayer::new(Event::Maghrib, date, config),
        Prayer::new(Event::Isha, date, config),
        Prayer::new(Event::Midnight, date, config),
    ]
}

pub fn list_prayers(config: &Config) -> [Prayer; 8] {
    list_prayers_for_date(config, Local::now().date_naive())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::madhab::Madhab;
    use crate::method::Method;

    fn paris_config() -> Config {
        use crate::arguments::Arguments;

        let args = Arguments {
            command: None,
            latitude: Some(48.8566),
            longitude: Some(2.3522),
            timezone: Some("Europe/Paris".to_string()),
            method: Some(Method::FranceUOIF),
            madhab: Some(Madhab::Shafi),
            fajr_mod: None,
            dhuhr_mod: None,
            asr_mod: None,
            maghrib_mod: None,
            isha_mod: None,
            notify_before: None,
            icon: None,
            urgency: None,
        };

        Config::new(&args)
    }

    fn makkah_config() -> Config {
        use crate::arguments::Arguments;

        let args = Arguments {
            command: None,
            latitude: Some(21.42664),
            longitude: Some(39.82563),
            timezone: Some("Asia/Riyadh".to_string()),
            method: Some(Method::UmmAlQura),
            madhab: Some(Madhab::Shafi),
            fajr_mod: None,
            dhuhr_mod: None,
            asr_mod: None,
            maghrib_mod: None,
            isha_mod: None,
            notify_before: None,
            icon: None,
            urgency: None,
        };

        Config::new(&args)
    }

    #[test]
    fn test_paris_prayer_times_october_2_2025() {
        let config = paris_config();
        let date = NaiveDate::from_ymd_opt(2025, 10, 2).unwrap();

        let prayers = list_prayers_for_date(&config, date);

        // curl -X GET "https://api.aladhan.com/v1/timings/02-10-2025?latitude=48.8566&longitude=2.3522&method=12&timezonestring=Europe%2FParis" -H 'accept: application/json' | jq
        assert_eq!(prayers[0].time().format("%H:%M").to_string(), "06:44");
        assert_eq!(prayers[1].time().format("%H:%M").to_string(), "07:52");
        assert_eq!(prayers[2].time().format("%H:%M").to_string(), "13:39");
        assert_eq!(prayers[3].time().format("%H:%M").to_string(), "16:48");
        assert_eq!(prayers[4].time().format("%H:%M").to_string(), "19:27");
        assert_eq!(prayers[5].time().format("%H:%M").to_string(), "19:27");
        assert_eq!(prayers[6].time().format("%H:%M").to_string(), "20:35");
        assert_eq!(prayers[7].time().format("%H:%M").to_string(), "01:39");
    }

    #[test]
    fn test_makkah_prayer_times_january_1_2022() {
        let config = makkah_config();
        let date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();

        let prayers = list_prayers_for_date(&config, date);

        // curl -X GET "https://api.aladhan.com/v1/timings/01-01-2022?latitude=21.42664&longitude=39.82563&method=4&timezonestring=Asia%2FRiyadh" -H 'accept: application/json' | jq
        assert_eq!(prayers[0].time().format("%H:%M").to_string(), "05:37");
        assert_eq!(prayers[1].time().format("%H:%M").to_string(), "06:59");
        assert_eq!(prayers[2].time().format("%H:%M").to_string(), "12:24");
        assert_eq!(prayers[3].time().format("%H:%M").to_string(), "15:29");
        assert_eq!(prayers[4].time().format("%H:%M").to_string(), "17:50");
        assert_eq!(prayers[5].time().format("%H:%M").to_string(), "17:50");
        assert_eq!(prayers[6].time().format("%H:%M").to_string(), "19:20");
        assert_eq!(prayers[7].time().format("%H:%M").to_string(), "00:24");
    }
}
