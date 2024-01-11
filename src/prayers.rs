mod calculations;

use self::calculations::{asr, equation_of_time as eot, solar_time_adjustment as sta};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
pub enum Method {
    Karachi,
    MuslimWorldLeague,
    Egyptian,
    UmmAlQura,
    NorthAmerica,
    French,
    Singapore,
    Russia,
    FixedInterval,
}
impl Default for Method {
    fn default() -> Self {
        Method::MuslimWorldLeague
    }
}

#[derive(Debug, Clone, EnumString)]
pub enum Madhab {
    Hanafi,
    Shafi,
}
impl Default for Madhab {
    fn default() -> Self {
        Madhab::Hanafi
    }
}

pub enum EnumPrayer {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
pub struct Prayer {
    enum_prayer: EnumPrayer,
    date: NaiveDateTime,
}
impl Prayer {
    pub fn date(&self) -> NaiveDateTime {
        self.date
    }
}

pub struct PrayersSchedule {
    date: NaiveDate,
    latitude: f64,
    longitude: f64,
}
impl PrayersSchedule {
    pub fn new(date: NaiveDate, latitude: f64, longitude: f64) -> Self {
        Self {
            date,
            latitude,
            longitude,
        }
    }

    pub fn get(&self, enum_prayer: EnumPrayer) -> Prayer {
        let time;

        let fajr_angle = 18.;
        let ishaa_angle = 17.;

        let object_shadow = 1;

        match enum_prayer {
            EnumPrayer::Fajr => {
                time = self.dhuhr() - sta(self.date, self.latitude, fajr_angle);
            }
            EnumPrayer::Dhuhr => {
                time = self.dhuhr();
            }
            EnumPrayer::Asr => {
                time = self.dhuhr() + asr(self.date, self.latitude, object_shadow);
            }
            EnumPrayer::Maghrib => {
                let sunset = self.dhuhr() + sta(self.date, self.latitude, 0.833);
                time = sunset;
            }
            EnumPrayer::Isha => {
                time = self.dhuhr() + sta(self.date, self.latitude, ishaa_angle);
            }
        }

        Prayer {
            enum_prayer,
            date: Self::to_naive_date_time(self.date, time),
        }
    }

    fn dhuhr(&self) -> f64 {
        let dhuhr = 12. + Self::timezone() as f64 - (self.longitude / 15.) - eot(self.date);
        dhuhr
    }

    fn to_naive_date_time(date: NaiveDate, time: f64) -> NaiveDateTime {
        // Create a NaiveTime instance
        let naive_time = NaiveTime::from_num_seconds_from_midnight_opt((time * 3600.) as u32, 0);
        // Create a NaiveDateTime instance
        NaiveDateTime::new(date, naive_time.unwrap())
    }

    fn timezone() -> i32 {
        Local::now().offset().local_minus_utc() / 3600
    }
}
