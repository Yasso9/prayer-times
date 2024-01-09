use std::path::PathBuf;

use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};
use notify_rust::Notification;

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
        .show();
    match notification {
        Ok(_) => println!("Notification sent"),
        Err(_) => println!("Failed to send notification"),
    }
}

fn main() -> Result<(), islam::Error> {
    println!("Starting Prayer Time Daemon");

    const METHOD: Method = Method::French;
    const MADHAB: Madhab = Madhab::Shafi;
    const SLEEPING_TIME: std::time::Duration = std::time::Duration::from_secs(30);

    let current_location = Location::new(49.049182, 2.035162);
    let config = Config::new().with(METHOD, MADHAB);
    let prayer_times = PrayerSchedule::new(current_location)?
        .with_config(config)
        .calculate()?;

    // Test notification
    // send_notification("Asr");

    let mut prayer = prayer_times.current();
    loop {
        println!("Checking prayer time...");

        let new_prayer = prayer_times.current();
        // Check if the prayer has changed
        if new_prayer != prayer {
            let prayer_name = new_prayer.name().unwrap_or_default();
            send_notification(&prayer_name);
            prayer = new_prayer;
        }

        println!("Sleeping...");
        std::thread::sleep(SLEEPING_TIME);
    }
}
