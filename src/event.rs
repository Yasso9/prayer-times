use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum Event {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Sunset,
    Midnight,
}
impl Event {
    pub fn list() -> [Event; 8] {
        use Event::*;
        [Fajr, Sunrise, Dhuhr, Asr, Sunset, Maghrib, Isha, Midnight]
    }
    pub fn previous(&self) -> Self {
        use Event::*;
        match self {
            Fajr => Isha,
            Sunrise => Fajr,
            Dhuhr => Sunrise,
            Asr => Dhuhr,
            Sunset => Asr,
            Maghrib => Asr,
            Isha => Maghrib,
            Midnight => Isha,
        }
    }
    pub fn next(&self) -> Event {
        use Event::*;
        match self {
            Fajr => Sunrise,
            Sunrise => Dhuhr,
            Dhuhr => Asr,
            Asr => Maghrib,
            Sunset => Isha,
            Maghrib => Isha,
            Isha => Midnight,
            Midnight => Fajr,
        }
    }
}
