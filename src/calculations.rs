use crate::{config::Config, event::Event};
use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, NaiveTime};

mod math {
    pub(crate) fn dcos(degrees: f64) -> f64 {
        degrees.to_radians().cos()
    }
    pub(crate) fn dsin(degrees: f64) -> f64 {
        degrees.to_radians().sin()
    }
    pub(crate) fn dtan(degrees: f64) -> f64 {
        degrees.to_radians().tan()
    }

    pub(crate) fn darcsin(x: f64) -> f64 {
        x.asin().to_degrees()
    }
    pub(crate) fn darccos(x: f64) -> f64 {
        x.acos().to_degrees()
    }
    pub(crate) fn darccot(x: f64) -> f64 {
        (1. / x).atan().to_degrees()
    }
    pub(crate) fn darctan2(y: f64, x: f64) -> f64 {
        y.atan2(x).to_degrees()
    }
}

fn fix(a: f64, b: f64) -> f64 {
    let result = a - b * (a / b).floor();
    if result < 0. {
        result + b
    } else {
        result
    }
}

// https://orbital-mechanics.space/reference/julian-date.html
fn to_julian_day(date: NaiveDate) -> f64 {
    let day = date.day() as f64;
    let month = date.month() as f64;
    let year = date.year() as f64;

    let a = (month - 14.) / 12.;
    let b = 1461. * (year + 4800. + a);
    let c = 367. * (month - 2. - (12. * a));
    let e = (year + 4900. + a) / 100.;

    (b / 4.) + (c / 12.) - (3. * e / 4.) + day - 32075.
}

// #[derive(PartialEq)]
#[derive(Clone)]
pub struct AstronomicalMeasures {
    date: NaiveDate,
    // a_sta: f64,
    // b_sta: f64,
    // dhuhr: f64,
    // asr: f64,
    fajr: f64,
    sunrise: f64,
    dhuhr: f64,
    asr: f64,
    sunset: f64,
    maghrib: f64,
    isha: f64,
    midnight: f64,
    third_of_night: f64,
}
impl AstronomicalMeasures {
    pub fn new(date: NaiveDate, config: &Config) -> Self {
        let (dos, eot) = {
            let jd = to_julian_day(date);

            let d = jd - 2451545.0;

            let g = fix(357.529 + 0.98560028 * d, 360.);
            let q = fix(280.459 + 0.98564736 * d, 360.);
            let l = fix(q + 1.915 * math::dsin(g) + 0.020 * math::dsin(2. * g), 360.);
            let e = 23.439 - 0.00000036 * d;
            let ra = math::darctan2(math::dcos(e) * math::dsin(l), math::dcos(l)) / 15.;

            // Declination of the Sun
            let dos = math::darcsin(math::dsin(e) * math::dsin(l));
            // Equation of Time
            let eot = q / 15. - fix(ra, 24.);

            (dos, eot)
        };

        let a_sta = math::dsin(config.lat()) * math::dsin(dos);
        let b_sta = math::dcos(config.lat()) * math::dcos(dos);
        let sta = |angle: f64| -> f64 {
            let a = -math::dsin(angle) - a_sta;
            let b = b_sta;
            1. / 15. * math::darccos(a / b)
        };

        let dhuhr = {
            let timezone = Local::now().offset().local_minus_utc() as f64 / 3600.;
            let a = 12. + timezone;
            let b = config.lon() / 15.;
            let c = eot;
            a - b - c
        };
        let asr = {
            let t = config.shadow_multiplier() as f64;
            let i = math::darccot(t + math::dtan(config.lat() - dos));
            let a = math::dsin(i) - math::dsin(config.lat()) * math::dsin(dos);
            let b = math::dcos(config.lat()) * math::dcos(dos);
            1. / 15. * math::darccos(a / b)
        };

        let sunrise = dhuhr - sta(0.833);
        let sunset = dhuhr + sta(0.833);
        let full_sunrise = if sunrise < sunset {
            sunrise + 24.
        } else {
            sunrise
        };
        let diff_night = full_sunrise - sunset;

        Self {
            date,
            fajr: dhuhr - sta(config.fajr_angle()) + config.offset(Event::Fajr),
            sunrise,
            dhuhr: dhuhr + config.offset(Event::Fajr),
            asr: dhuhr + asr + config.offset(Event::Asr),
            maghrib: sunset + config.offset(Event::Maghrib),
            sunset: dhuhr + sta(0.833),
            isha: dhuhr + sta(config.isha_angle()) + config.offset(Event::Isha),
            midnight: sunset + 0.5 * diff_night,
            third_of_night: sunset + 0.75 * diff_night,
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    fn get_raw_time(&self, event: Event) -> f64 {
        match event {
            Event::Fajr => self.fajr,
            Event::Sunrise => self.sunrise,
            Event::Dhuhr => self.dhuhr,
            Event::Asr => self.asr,
            Event::Sunset => self.sunset,
            Event::Maghrib => self.maghrib,
            Event::Isha => self.isha,
            Event::Midnight => self.midnight,
            Event::Qiyam => self.third_of_night,
        }
    }

    pub fn date_time(&self, event: Event) -> NaiveDateTime {
        let time = self.get_raw_time(event);

        let naive_time =
            NaiveTime::from_num_seconds_from_midnight_opt((time.rem_euclid(24.) * 3600.) as u32, 0);

        let time_shift = (time / 24.).floor();
        let date = if time_shift >= 1. {
            self.date()
                .checked_add_days(Days::new(time_shift as u64))
                .unwrap()
        } else if time_shift < 0. {
            self.date()
                .checked_sub_days(Days::new(-time_shift as u64))
                .unwrap()
        } else {
            self.date()
        };
        // TODO: do not have an expect here
        NaiveDateTime::new(date, naive_time.expect("Error in prayer calculation"))
    }
}
