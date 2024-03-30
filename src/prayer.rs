use crate::calculations::{asr, equation_of_time, solar_time_adjustment};
use crate::event::Event;
use crate::Config;
use chrono::{Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn get_prayer(enum_prayer: Event, date: NaiveDate, config: &Config) -> Prayer {
    // Time of dhuhr. Used to calculate all other prayers
    fn dhuhr(date: NaiveDate, config: &Config) -> f64 {
        let timezone = Local::now().offset().local_minus_utc() / 3600;
        let a = 12 + timezone;
        let b = config.lon() / 15.;
        let c = equation_of_time(date);
        a as f64 - b - c
    }
    fn to_naive_date_time(date: NaiveDate, time: f64) -> NaiveDateTime {
        let naive_time = NaiveTime::from_num_seconds_from_midnight_opt((time * 3600.) as u32, 0);
        NaiveDateTime::new(date, naive_time.expect("Error in prayer calculation"))
    }

    let dhuhr = dhuhr(date, config);

    let time = match enum_prayer {
        Event::Fajr => {
            dhuhr - solar_time_adjustment(date, config.lat(), config.fajr()) + config.fajr_offset()
        }
        Event::Dhuhr => dhuhr + config.dhuhr_offset(),
        Event::Asr => {
            dhuhr + asr(date, config.lat(), config.shadow_multiplier()) + config.asr_offset()
        }
        Event::Maghrib => {
            let sunset = dhuhr + solar_time_adjustment(date, config.lat(), 0.833);
            sunset + config.maghrib_offset()
        }
        Event::Isha => {
            dhuhr + solar_time_adjustment(date, config.lat(), config.isha()) + config.isha_offset()
        }
    };

    Prayer {
        event: enum_prayer,
        date: to_naive_date_time(date, time),
    }
}

#[derive(PartialEq)]
pub struct Prayer {
    event: Event,
    date: NaiveDateTime,
}
impl Prayer {
    pub fn event(&self) -> Event {
        self.event
    }
    pub fn date_time(&self) -> NaiveDateTime {
        self.date
    }

    pub fn previous(&self, config: &Config) -> Prayer {
        let previous_prayer = get_prayer(self.event.previous(), self.date.date(), config);
        if previous_prayer.date_time().time() <= self.date_time().time() {
            return previous_prayer;
        }

        let previous_date = self
            .date_time()
            .date()
            .checked_sub_days(Days::new(1))
            .unwrap();
        get_prayer(self.event.previous(), previous_date, config)
    }

    pub fn next(&self, config: &Config) -> Prayer {
        let next_prayer = get_prayer(self.event.next(), self.date.date(), config);
        if next_prayer.date_time().time() >= self.date_time().time() {
            return next_prayer;
        }

        let next_date = self
            .date_time()
            .date()
            .checked_add_days(Days::new(1))
            .unwrap();
        get_prayer(self.event.next(), next_date, config)
    }

    // Returns the time remaining for the next prayer to happen
    pub fn time_remaining(&self) -> Duration {
        let duration = self
            .date_time()
            .signed_duration_since(Local::now().naive_local());

        // The time remaining should not be negative
        if duration < Duration::zero() {
            return Duration::zero();
        }
        duration
    }

    // Returns true if the time of the prayer passed
    pub fn time_has_passed(&self) -> bool {
        self.time_remaining() <= Duration::zero()
    }

    pub fn text_duration(&self) -> String {
        let time_remaining = self.time_remaining();
        format!(
            "Adhan {} in {:02}:{:02}",
            self.event(),
            time_remaining.num_hours(),
            time_remaining.num_minutes() % 60
        )
    }
    pub fn text_time(&self) -> String {
        format!("Adhan {} at {}", self.event(), self.date_time().time())
    }
}
