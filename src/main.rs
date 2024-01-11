mod cli;
mod prayers;

use chrono::Local;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

use crate::prayers::{EnumPrayer, PrayersSchedule};

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

fn main() {
    println!("Starting Prayer Time Daemon");

    let args = crate::cli::parse_cli();
    println!("name: {:?}", args);

    const SLEEPING_TIME: std::time::Duration = std::time::Duration::from_secs(30);

    let prayers = PrayersSchedule::new(Local::now().date_naive(), 49.049182, 2.035162);
    println!("Fajr: {}", prayers.get(EnumPrayer::Fajr).date());
    println!("Dhuhr: {}", prayers.get(EnumPrayer::Dhuhr).date());
    println!("Asr: {}", prayers.get(EnumPrayer::Asr).date());
    println!("Maghrib: {}", prayers.get(EnumPrayer::Maghrib).date());
    println!("Isha: {}", prayers.get(EnumPrayer::Isha).date());

    // Test notification
    send_notification("Asr");

    loop {
        println!("Sleeping...");
        std::thread::sleep(SLEEPING_TIME);
    }
}
