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
    pub fn list() -> [Event; 5] {
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
