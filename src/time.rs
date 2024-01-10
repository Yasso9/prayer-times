#[allow(dead_code)]
pub mod prayer_time {
    use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

    fn to_naive_date_time(date: NaiveDate, time: f64) -> NaiveDateTime {
        // Extract the whole number part as hours
        let hours = time.floor() as u32;
        // Extract the fractional part and convert it to minutes
        let minutes = ((time - hours as f64) * 60.0).round() as u32;

        // Create a NaiveTime instance
        let naive_time = NaiveTime::from_hms_opt(hours, minutes, 0);
        // Create a NaiveDateTime instance
        NaiveDateTime::new(date, naive_time.unwrap())
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

    pub struct Prayers {
        date: NaiveDate,
        latitude: f64,
        longitude: f64,
    }
    impl Prayers {
        pub fn new(date: NaiveDate, latitude: f64, longitude: f64) -> Self {
            Self {
                date,
                latitude,
                longitude,
            }
        }

        pub fn get(&self, enum_prayer: EnumPrayer) -> Prayer {
            match enum_prayer {
                EnumPrayer::Dhuhr => Prayer {
                    enum_prayer: EnumPrayer::Dhuhr,
                    date: self.dhuhr(),
                },
                _ => panic!("Prayer not handled"),
            }
        }

        fn dhuhr(&self) -> NaiveDateTime {
            let timezone_offset = Local::now().offset().local_minus_utc() / 3600;
            println!("Timezone offset: {}", timezone_offset);
            let dhuhr = 13. - (self.longitude / 15.) - self.equation_of_time();

            to_naive_date_time(self.date, dhuhr)
        }

        fn julian_date(&self) -> u32 {
            let day = self.date.day();
            let month = self.date.month();
            let year = self.date.year() as u32;

            let a = (14 - month) / 12;
            let y = year + 4800 - a;
            let m = month + 12 * a - 3;

            let jd = day + (153 * m + 2) / 5 + y * 365 + y / 4 - y / 100 + y / 400 - 32045;

            println!("julian_date: {}", jd);

            jd
        }

        fn equation_of_time(&self) -> f64 {
            let d = self.julian_date() as f64 - 2451545.;

            let g = 357.529 + 0.98560028 * d;
            let q = 280.459 + 0.98564736 * d;
            let L = q + 1.915 * g.sin() + 0.020 * (2. * g).sin();

            let R = 1.00014 - 0.01671 * g.cos() - 0.00014 * (2. * g).cos();
            let e = 23.439 - 0.00000036 * d;
            let RA = (e.cos() * L.sin()).atan2(L.cos()) / 15.;

            let declination_of_sun = (e.sin() * L.sin()).asin(); // declination of the Sun
            let equation_of_time = -7.31 / 60.;
            // let equation_of_time = q / 15. - RA;

            println!("equation_of_time: {}", equation_of_time);

            equation_of_time
        }
    }
}
