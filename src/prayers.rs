mod arguments;
mod calculations;

use self::{
    arguments::Config,
    calculations::{asr, equation_of_time as eot, solar_time_adjustment as sta},
};
use chrono::{Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum EnumPrayer {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
impl EnumPrayer {
    fn list() -> [EnumPrayer; 5] {
        [
            EnumPrayer::Fajr,
            EnumPrayer::Dhuhr,
            EnumPrayer::Asr,
            EnumPrayer::Maghrib,
            EnumPrayer::Isha,
        ]
    }
    pub fn previous(&self) -> EnumPrayer {
        match self {
            EnumPrayer::Fajr => EnumPrayer::Isha,
            EnumPrayer::Dhuhr => EnumPrayer::Fajr,
            EnumPrayer::Asr => EnumPrayer::Dhuhr,
            EnumPrayer::Maghrib => EnumPrayer::Asr,
            EnumPrayer::Isha => EnumPrayer::Maghrib,
        }
    }
    pub fn next(&self) -> EnumPrayer {
        match self {
            EnumPrayer::Fajr => EnumPrayer::Dhuhr,
            EnumPrayer::Dhuhr => EnumPrayer::Asr,
            EnumPrayer::Asr => EnumPrayer::Maghrib,
            EnumPrayer::Maghrib => EnumPrayer::Isha,
            EnumPrayer::Isha => EnumPrayer::Fajr,
        }
    }
}
pub struct Prayer {
    enum_prayer: EnumPrayer,
    date: NaiveDateTime,
}
impl Prayer {
    pub fn date_time(&self) -> NaiveDateTime {
        self.date
    }
    pub fn enum_prayer(&self) -> EnumPrayer {
        self.enum_prayer
    }

    pub fn previous(&self) -> Prayer {
        let previous_prayer = get_prayer(self.enum_prayer.previous(), self.date.date());
        if previous_prayer.date_time().date() == self.date_time().date() {
            return previous_prayer;
        }

        let previous_date = self
            .date_time()
            .date()
            .checked_sub_days(Days::new(1))
            .unwrap();
        return get_prayer(self.enum_prayer.previous(), previous_date);
    }

    pub fn next(&self) -> Prayer {
        let next_prayer = get_prayer(self.enum_prayer.next(), self.date.date());
        // println!(
        //     "Next 1st: {} {}",
        //     next_prayer.enum_prayer(),
        //     next_prayer.date_time()
        // );
        if next_prayer.date_time().time() >= self.date_time().time() {
            return next_prayer;
        }

        let next_date = self
            .date_time()
            .date()
            .checked_add_days(Days::new(1))
            .unwrap();
        return get_prayer(self.enum_prayer.next(), next_date);
    }

    // Returns the time remaining for the next prayer to happen
    pub fn time_remaining(&self) -> Duration {
        let duration = self
            .date_time()
            .signed_duration_since(Local::now().naive_local());

        // println!("Next prayer : {} {}", self.enum_prayer(), self.date_time());
        // println!("Current time: {}", Local::now().naive_local());
        // println!("Duration: {:?}", duration);
        // println!("Duration Hour: {}", duration.num_hours());

        if duration < Duration::zero() {
            return Duration::zero();
        }
        duration
    }
}

pub fn get_prayer(enum_prayer: EnumPrayer, date: NaiveDate) -> Prayer {
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
        EnumPrayer::Fajr => {
            time = dhuhr - sta(date, config.lat(), config.fajr()) + config.fajr_offset();
        }
        EnumPrayer::Dhuhr => {
            time = dhuhr + config.dhuhr_offset();
        }
        EnumPrayer::Asr => {
            time =
                dhuhr + asr(date, config.lat(), config.shadow_multiplier()) + config.asr_offset();
        }
        EnumPrayer::Maghrib => {
            let sunset = dhuhr + sta(date, config.lat(), 0.833);
            time = sunset + config.maghrib_offset();
        }
        EnumPrayer::Isha => {
            time = dhuhr + sta(date, config.lat(), config.isha()) + config.isha_offset();
        }
    }

    Prayer {
        enum_prayer,
        date: to_naive_date_time(date, time),
    }
}

pub fn current() -> Prayer {
    next().previous()
}
pub fn next() -> Prayer {
    let current_date = Local::now().date_naive();
    let current_time = Local::now().time();

    for enum_prayer in EnumPrayer::list() {
        let prayer = get_prayer(enum_prayer, current_date);
        if current_time <= prayer.date_time().time() {
            return prayer;
        }
    }

    get_prayer(EnumPrayer::Isha, current_date).next()
}
pub fn list_prayers() -> [Prayer; 5] {
    let date = Local::now().date_naive();

    [
        get_prayer(EnumPrayer::Fajr, date),
        get_prayer(EnumPrayer::Dhuhr, date),
        get_prayer(EnumPrayer::Asr, date),
        get_prayer(EnumPrayer::Maghrib, date),
        get_prayer(EnumPrayer::Isha, date),
    ]
}
