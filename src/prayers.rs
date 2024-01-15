pub mod arguments;
mod calculations;

use self::{
    arguments::Config,
    calculations::{asr, equation_of_time as eot, solar_time_adjustment as sta},
};
use chrono::{Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum Event {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
impl Event {
    fn list() -> [Event; 5] {
        [
            Event::Fajr,
            Event::Dhuhr,
            Event::Asr,
            Event::Maghrib,
            Event::Isha,
        ]
    }
    pub fn previous(&self) -> Event {
        match self {
            Event::Fajr => Event::Isha,
            Event::Dhuhr => Event::Fajr,
            Event::Asr => Event::Dhuhr,
            Event::Maghrib => Event::Asr,
            Event::Isha => Event::Maghrib,
        }
    }
    pub fn next(&self) -> Event {
        match self {
            Event::Fajr => Event::Dhuhr,
            Event::Dhuhr => Event::Asr,
            Event::Asr => Event::Maghrib,
            Event::Maghrib => Event::Isha,
            Event::Isha => Event::Fajr,
        }
    }
}
pub struct Prayer {
    event: Event,
    date: NaiveDateTime,
}
impl Prayer {
    pub fn date_time(&self) -> NaiveDateTime {
        self.date
    }
    pub fn event(&self) -> Event {
        self.event
    }

    pub fn previous(&self) -> Prayer {
        let previous_prayer = get_prayer(self.event.previous(), self.date.date());
        if previous_prayer.date_time().time() <= self.date_time().time() {
            return previous_prayer;
        }

        let previous_date = self
            .date_time()
            .date()
            .checked_sub_days(Days::new(1))
            .unwrap();
        return get_prayer(self.event.previous(), previous_date);
    }

    pub fn next(&self) -> Prayer {
        let next_prayer = get_prayer(self.event.next(), self.date.date());
        if next_prayer.date_time().time() >= self.date_time().time() {
            return next_prayer;
        }

        let next_date = self
            .date_time()
            .date()
            .checked_add_days(Days::new(1))
            .unwrap();
        return get_prayer(self.event.next(), next_date);
    }

    // Returns the time remaining for the next prayer to happen
    pub fn time_remaining(&self) -> Duration {
        let duration = self
            .date_time()
            .signed_duration_since(Local::now().naive_local());

        if duration < Duration::zero() {
            return Duration::zero();
        }
        duration
    }
}

pub fn get_prayer(enum_prayer: Event, date: NaiveDate) -> Prayer {
    fn dhuhr(date: NaiveDate, config: &Config) -> f64 {
        let timezone = Local::now().offset().local_minus_utc() / 3600;

        let a = 12 + timezone;
        let b = config.lon() / 15.;
        let c = eot(date);
        a as f64 - b - c
    }
    fn to_naive_date_time(date: NaiveDate, time: f64) -> NaiveDateTime {
        // Create a NaiveTime instance
        let naive_time = NaiveTime::from_num_seconds_from_midnight_opt((time * 3600.) as u32, 0);
        // Create a NaiveDateTime instance
        NaiveDateTime::new(date, naive_time.expect("Error in prayer calculation"))
    }

    let config = Config::new();
    let dhuhr = dhuhr(date, &config);

    let time;

    match enum_prayer {
        Event::Fajr => {
            time = dhuhr - sta(date, config.lat(), config.fajr()) + config.fajr_offset();
        }
        Event::Dhuhr => {
            time = dhuhr + config.dhuhr_offset();
        }
        Event::Asr => {
            time =
                dhuhr + asr(date, config.lat(), config.shadow_multiplier()) + config.asr_offset();
        }
        Event::Maghrib => {
            let sunset = dhuhr + sta(date, config.lat(), 0.833);
            time = sunset + config.maghrib_offset();
        }
        Event::Isha => {
            time = dhuhr + sta(date, config.lat(), config.isha()) + config.isha_offset();
        }
    }

    Prayer {
        event: enum_prayer,
        date: to_naive_date_time(date, time),
    }
}

pub fn current() -> Prayer {
    next().previous()
}
pub fn next() -> Prayer {
    let current_date = Local::now().date_naive();
    let current_time = Local::now().time();

    for enum_prayer in Event::list() {
        let prayer = get_prayer(enum_prayer, current_date);
        if current_time <= prayer.date_time().time() {
            return prayer;
        }
    }

    // Time of fajr of tomorrow
    get_prayer(Event::Isha, current_date).next()
}
pub fn list_prayers() -> [Prayer; 5] {
    let date = Local::now().date_naive();

    [
        get_prayer(Event::Fajr, date),
        get_prayer(Event::Dhuhr, date),
        get_prayer(Event::Asr, date),
        get_prayer(Event::Maghrib, date),
        get_prayer(Event::Isha, date),
    ]
}
