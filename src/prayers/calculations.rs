use chrono::{Datelike, NaiveDate};

fn fix(a: f64, b: f64) -> f64 {
    let result = a - b * (a / b).floor();
    if result < 0. {
        result + b
    } else {
        result
    }
}

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

struct DayValues {
    dos: f64,
    eot: f64,
}
impl DayValues {
    fn new(date: NaiveDate) -> DayValues {
        let jd = to_julian_day(date);

        let d = jd - 2451545.0;

        let g = fix(357.529 + 0.98560028 * d, 360.);
        let q = fix(280.459 + 0.98564736 * d, 360.);
        let l = fix(q + 1.915 * math::dsin(g) + 0.020 * math::dsin(2. * g), 360.);
        let e = 23.439 - 0.00000036 * d;
        let ra = math::darctan2(math::dcos(e) * math::dsin(l), math::dcos(l)) / 15.;

        let dos = math::darcsin(math::dsin(e) * math::dsin(l));
        let eot = q / 15. - fix(ra, 24.);
        DayValues { dos, eot }
    }
}

pub fn asr(date: NaiveDate, latitude: f64, object_shadow_multiplier: u32) -> f64 {
    let dos = DayValues::new(date).dos;

    let t = object_shadow_multiplier as f64;
    let i = math::darccot(t + math::dtan(latitude - dos));
    let a = math::dsin(i) - math::dsin(latitude) * math::dsin(dos);
    let b = math::dcos(latitude) * math::dcos(dos);

    let sta = 1. / 15. * math::darccos(a / b);
    sta
}

pub fn solar_time_adjustment(date: NaiveDate, latitude: f64, angle: f64) -> f64 {
    let dos = DayValues::new(date).dos;

    let a = -math::dsin(angle) - math::dsin(latitude) * math::dsin(dos);
    let b = math::dcos(latitude) * math::dcos(dos);

    let sta = 1. / 15. * math::darccos(a / b);
    sta
}

pub fn equation_of_time(date: NaiveDate) -> f64 {
    DayValues::new(date).eot
}
