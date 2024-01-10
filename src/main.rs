mod time;
use crate::time::prayer_time::{EnumPrayer, Prayers};
use chrono::{Local, NaiveDate};
use clap::Parser;

use std::path::PathBuf;

use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};
use notify_rust::{Notification, Urgency};

// Get the icon of the notification that should be sent
fn get_icon() -> Result<PathBuf, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    Ok(current_dir.join("assets").join("mosque-svgrepo-com.png"))
}

// Send notification to the screen
fn send_notification(prayer_name: &str) {
    let notification = Notification::new()
        .summary(&format!("Adhan {prayer_name}"))
        .icon(get_icon().unwrap_or_default().to_str().unwrap_or_default())
        .urgency(Urgency::Critical)
        .show();
    match notification {
        Ok(_) => println!("Notification sent"),
        Err(_) => println!("Failed to send notification"),
    }
}

#[derive(Debug, Clone)]
enum PrayerMethod {
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
impl Default for PrayerMethod {
    fn default() -> Self {
        PrayerMethod::MuslimWorldLeague
    }
}
impl From<&str> for PrayerMethod {
    fn from(value: &str) -> Self {
        match value {
            "Karachi" => PrayerMethod::Karachi,
            "MuslimWorldLeague" => PrayerMethod::MuslimWorldLeague,
            "Egyptian" => PrayerMethod::Egyptian,
            "UmmAlQura" => PrayerMethod::UmmAlQura,
            "NorthAmerica" => PrayerMethod::NorthAmerica,
            "French" => PrayerMethod::French,
            "Singapore" => PrayerMethod::Singapore,
            "Russia" => PrayerMethod::Russia,
            "FixedInterval" => PrayerMethod::FixedInterval,
            _ => {
                println!(
                    "Invalid method: {value}, using default: {:?}",
                    PrayerMethod::default()
                );
                PrayerMethod::default()
            }
        }
    }
}
impl From<PrayerMethod> for Method {
    fn from(prayer_method: PrayerMethod) -> Self {
        match prayer_method {
            PrayerMethod::Karachi => Method::Karachi,
            PrayerMethod::MuslimWorldLeague => Method::MuslimWorldLeague,
            PrayerMethod::Egyptian => Method::Egyptian,
            PrayerMethod::UmmAlQura => Method::UmmAlQura,
            PrayerMethod::NorthAmerica => Method::NorthAmerica,
            PrayerMethod::French => Method::French,
            PrayerMethod::Singapore => Method::Singapore,
            PrayerMethod::Russia => Method::Russia,
            PrayerMethod::FixedInterval => Method::FixedInterval,
        }
    }
}
#[derive(Debug, Clone)]
enum PrayerMadhab {
    Hanafi,
    Shafi,
}
impl Default for PrayerMadhab {
    fn default() -> Self {
        PrayerMadhab::Hanafi
    }
}
impl From<&str> for PrayerMadhab {
    fn from(value: &str) -> Self {
        match value {
            "Hanafi" => PrayerMadhab::Hanafi,
            "Shafi" => PrayerMadhab::Shafi,
            _ => {
                println!(
                    "Invalid method: {value}, using default: {:?}",
                    PrayerMadhab::default()
                );
                PrayerMadhab::default()
            }
        }
    }
}
impl From<PrayerMadhab> for Madhab {
    fn from(madhab: PrayerMadhab) -> Self {
        match madhab {
            PrayerMadhab::Hanafi => Madhab::Hanafi,
            PrayerMadhab::Shafi => Madhab::Shafi,
        }
    }
}

/// Program to notify prayer times
#[derive(Parser, Debug)]
#[command(name = "prayer-time")]
// Read from `Cargo.toml`
#[command(author, version, about, long_about = None)]
struct Args {
    /// Calculation Method to use
    #[arg(short = 'm', long, default_value = "MuslimWorldLeague")]
    method: PrayerMethod,
    #[arg(short = 'M', long, default_value = "Hanafi")]
    madhab: PrayerMadhab,

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

fn main() {
    println!("Starting Prayer Time Daemon");

    let args = Args::parse();
    println!("name: {:?}", args);

    let method: Method = Method::from(args.method);
    let madhab: Madhab = Madhab::from(args.madhab);
    const SLEEPING_TIME: std::time::Duration = std::time::Duration::from_secs(30);

    let current_location = Location::new(49.049182, 2.035162);
    let config = Config::new().with(method, madhab);
    let prayer_times = PrayerSchedule::new(current_location)
        .expect("Failed to create prayer times. Maybe wrong location?")
        .with_config(config)
        .calculate()
        .expect("Failed to calculate prayer times");

    // Test notification
    // send_notification("Asr");

    let prayers = Prayers::new(Local::now().date_naive(), 49.049182, 2.035162);
    println!("Dhuhr: {}", prayers.get(EnumPrayer::Dhuhr).date());

    // let mut prayer = prayer_times.current();
    // loop {
    //     println!("Checking prayer time...");

    //     let new_prayer = prayer_times.current();
    //     // Check if the prayer has changed
    //     if new_prayer != prayer {
    //         let prayer_name = new_prayer.name().unwrap_or_default();
    //         send_notification(&prayer_name);
    //         prayer = new_prayer;
    //     }

    //     println!("Sleeping...");
    //     std::thread::sleep(SLEEPING_TIME);
    // }
}
