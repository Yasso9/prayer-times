use clap::Parser;
use strum_macros::EnumString;

/// Program to notify prayer times
#[derive(Parser, Debug)]
#[command(name = "prayer-time")]
// Read from `Cargo.toml`
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Latitude. Defaults to the current location
    #[arg(short = 'l', long)]
    latitude: f64,
    /// Longitude. Defaults to the current location
    #[arg(short = 'L', long)]
    longitude: f64,

    /// Calculation Method to use
    #[arg(short = 'm', long, default_value = "MuslimWorldLeague")]
    method: Method,
    /// Madhab to use
    #[arg(short = 'M', long, default_value = "Shafi")]
    madhab: Madhab,

    /// Minutes to add or remove to the Fajr time
    #[arg(long, default_value_t = 0)]
    fajr_mod: i8,
    /// Minutes to add or remove to the Dohr time
    #[arg(long, default_value_t = 0)]
    dohr_mod: i8,
    /// Minutes to add or remove to the Asr time
    #[arg(long, default_value_t = 0)]
    asr_mod: i8,
    /// Minutes to add or remove to the Maghrib time
    #[arg(long, default_value_t = 0)]
    maghrib_mod: i8,
    /// Minutes to add or remove to the Isha time
    #[arg(long, default_value_t = 0)]
    isha_mod: i8,
}

struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Clone, EnumString)]
pub enum Method {
    MuslimWorldLeague,
    NorthAmerica,
    Egyptian,
    UmmAlQura,
    Karachi,
    Tehran,
    Jafari,
    FranceUOIF,
    FranceGMP,
    // Gulf,
    // Kuwait,
    // Qatar,
    // Singapore,
    // France18,
    // Turkey,
    // Russia,
    // Tunisia,
    // Algeria,
    // Indonesia,
    // Morocco,
    // Lisboa,
}
impl Method {
    pub fn fajr_angle(&self) -> f64 {
        match self {
            Method::MuslimWorldLeague => 18.,
            Method::NorthAmerica => 15.,
            Method::Egyptian => 19.5,
            Method::UmmAlQura => 18.5,
            Method::Karachi => 18.,
            Method::Tehran => 17.7,
            Method::Jafari => 16.,
            Method::FranceUOIF => 12.,
            Method::FranceGMP => 18.,
        }
    }
    pub fn isha_angle(&self) -> f64 {
        match self {
            Method::MuslimWorldLeague => 17.,
            Method::NorthAmerica => 15.,
            Method::Egyptian => 17.5,
            Method::UmmAlQura => 18.5, // Wrong
            Method::Karachi => 18.,
            Method::Tehran => 14.,
            Method::Jafari => 14.,
            Method::FranceUOIF => 12.,
            Method::FranceGMP => 18.,
        }
    }
    // Set maghrib and midnight
}
impl Default for Method {
    fn default() -> Self {
        Method::MuslimWorldLeague
    }
}

#[derive(Debug, Clone, EnumString)]
pub enum Madhab {
    Shafi,
    Hanafi,
}
impl Madhab {
    pub fn shadow_multiplier(&self) -> u8 {
        match self {
            Madhab::Shafi => 1,
            Madhab::Hanafi => 2,
        }
    }
}
impl Default for Madhab {
    fn default() -> Self {
        Madhab::Shafi
    }
}
pub struct Config {
    location: Location,
    method: Method,
    madhab: Madhab,
    time_mod: [i8; 5],
}
impl Config {
    pub fn new() -> Self {
        let args = Arguments::parse();
        println!("{:?}", args);
        Self {
            location: Location {
                lat: args.latitude,
                lon: args.longitude,
            },
            method: args.method,
            madhab: args.madhab,
            time_mod: [
                args.fajr_mod,
                args.dohr_mod,
                args.asr_mod,
                args.maghrib_mod,
                args.isha_mod,
            ],
        }
    }

    pub fn lat(&self) -> f64 {
        self.location.lat
    }
    pub fn lon(&self) -> f64 {
        self.location.lon
    }

    pub fn fajr(&self) -> f64 {
        self.method.fajr_angle()
    }
    pub fn isha(&self) -> f64 {
        self.method.isha_angle()
    }
    pub fn shadow_multiplier(&self) -> u8 {
        self.madhab.shadow_multiplier()
    }

    pub fn fajr_offset(&self) -> f64 {
        self.time_mod[0] as f64 / 60.
    }
    pub fn dhuhr_offset(&self) -> f64 {
        self.time_mod[1] as f64 / 60.
    }
    pub fn asr_offset(&self) -> f64 {
        self.time_mod[2] as f64 / 60.
    }
    pub fn maghrib_offset(&self) -> f64 {
        self.time_mod[3] as f64 / 60.
    }
    pub fn isha_offset(&self) -> f64 {
        self.time_mod[4] as f64 / 60.
    }
}
