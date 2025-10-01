use crate::{config::Config, event::Event};
use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

mod math;

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

#[derive(Clone)]
pub struct AstronomicalMeasures {
    date: NaiveDate,
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
        // https://praytimes.org/calculation#astronomical_measures
        let (declination_of_sun, equation_of_time) = {
            let julian_day = to_julian_day(date);

            let d = julian_day - 2451545.0;

            let g = fix(357.529 + 0.98560028 * d, 360.);
            let q = fix(280.459 + 0.98564736 * d, 360.);
            let l = fix(q + 1.915 * math::dsin(g) + 0.020 * math::dsin(2. * g), 360.);
            let e = 23.439 - 0.00000036 * d;
            let ra = math::darctan2(math::dcos(e) * math::dsin(l), math::dcos(l)) / 15.;

            let declination_of_sun = math::darcsin(math::dsin(e) * math::dsin(l));
            let equation_of_time = q / 15. - fix(ra, 24.);

            (declination_of_sun, equation_of_time)
        };

        let solar_hour_angle = |angle: f64| -> f64 {
            let numerator =
                -math::dsin(angle) - math::dsin(config.lat()) * math::dsin(declination_of_sun);
            let denominator = math::dcos(config.lat()) * math::dcos(declination_of_sun);
            1. / 15. * math::darccos(numerator / denominator)
        };

        // https://praytimes.org/calculation#dhuhr
        let dhuhr = {
            let timezone = Local
                .offset_from_local_date(&date)
                .single()
                .unwrap()
                .local_minus_utc() as f64
                / 3600.;
            let a = 12. + timezone;
            let b = config.lon() / 15.;
            let c = equation_of_time;
            a - b - c
        };
        // https://praytimes.org/calculation#asr
        let asr = {
            let t = config.shadow_multiplier() as f64;
            let i = math::darccot(t + math::dtan((config.lat() - declination_of_sun).abs()));
            let a = math::dsin(i) - math::dsin(config.lat()) * math::dsin(declination_of_sun);
            let b = math::dcos(config.lat()) * math::dcos(declination_of_sun);
            1. / 15. * math::darccos(a / b)
        };

        // https://praytimes.org/calculation#sunrisesunset
        let sunrise = dhuhr - solar_hour_angle(0.833);
        let sunset = dhuhr + solar_hour_angle(0.833);
        let full_sunrise = if sunrise < sunset {
            sunrise + 24.
        } else {
            sunrise
        };
        let diff_night = full_sunrise - sunset;

        Self {
            date,
            fajr: dhuhr - solar_hour_angle(config.fajr_angle()) + config.offset(Event::Fajr),
            sunrise,
            dhuhr: dhuhr + config.offset(Event::Dhuhr),
            asr: dhuhr + asr + config.offset(Event::Asr),
            maghrib: sunset + config.offset(Event::Maghrib),
            sunset: dhuhr + solar_hour_angle(0.833),
            isha: dhuhr + solar_hour_angle(config.isha_angle()) + config.offset(Event::Isha),
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
