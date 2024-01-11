use clap::Parser;

use crate::prayers::{Madhab, Method};

/// Program to notify prayer times
#[derive(Parser, Debug)]
#[command(name = "prayer-time")]
// Read from `Cargo.toml`
#[command(author, version, about, long_about = None)]
pub struct PrayerCliArgs {
    /// Calculation Method to use
    #[arg(short = 'm', long, default_value = "MuslimWorldLeague")]
    method: Method,
    #[arg(short = 'M', long, default_value = "Hanafi")]
    madhab: Madhab,

    /// Minutes to add or remove to the Fajr time
    #[arg(long, default_value_t = 0)]
    fajr_mod: u8,
    /// Minutes to add or remove to the Dohr time
    #[arg(long, default_value_t = 0)]
    dohr_mod: u8,
    /// Minutes to add or remove to the Asr time
    #[arg(long, default_value_t = 0)]
    asr_mod: u8,
    /// Minutes to add or remove to the Maghrib time
    #[arg(long, default_value_t = 0)]
    maghrib_mod: u8,
    /// Minutes to add or remove to the Isha time
    #[arg(long, default_value_t = 0)]
    isha_mod: u8,
}

pub fn parse_cli() -> PrayerCliArgs {
    PrayerCliArgs::parse()
}
