use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum Event {
    Fajr,
    Shourouk,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
impl Event {
    pub fn list() -> [Event; 6] {
        [
            Event::Fajr,
            Event::Shourouk,
            Event::Dhuhr,
            Event::Asr,
            Event::Maghrib,
            Event::Isha,
        ]
    }
    pub fn previous(&self) -> Event {
        match self {
            Event::Fajr => Event::Isha,
            Event::Shourouk => Event::Fajr,
            Event::Dhuhr => Event::Shourouk,
            Event::Asr => Event::Dhuhr,
            Event::Maghrib => Event::Asr,
            Event::Isha => Event::Maghrib,
        }
    }
    pub fn next(&self) -> Event {
        match self {
            Event::Fajr => Event::Shourouk,
            Event::Shourouk => Event::Dhuhr,
            Event::Dhuhr => Event::Asr,
            Event::Asr => Event::Maghrib,
            Event::Maghrib => Event::Isha,
            Event::Isha => Event::Fajr,
        }
    }
}
