use crate::config::Config;
use crate::prayer::Prayer;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

// Get the icon of the notification that should be sent
fn get_icon() -> Result<PathBuf, std::io::Error> {
    let asset_path;

    if cfg!(debug_assertions) {
        let current_dir = std::env::current_dir()?;
        asset_path = current_dir.join("assets");
    } else {
        asset_path = PathBuf::from("/usr/share/icons");
    }
    Ok(asset_path.join("mosque-svgrepo-com.png"))
}

fn send_notification(summary: String, urgency: Urgency) {
    let mut notification = Notification::new(); // so the notification will live
    let notification = notification.summary(&summary).urgency(urgency);

    match get_icon() {
        Ok(icon_path) => match icon_path.to_str() {
            Some(icon_path) => {
                notification.icon(icon_path);
            }
            None => println!("Failed to get icon path"),
        },
        Err(_) => println!("Failed to get icon path"),
    }

    match notification.show() {
        Ok(_) => println!("Notification sent"),
        Err(_) => println!("Failed to send notification"),
    }
}

pub fn notify_prayer(prayer: &Prayer, config: &Config) {
    let summary = format!("Adhan {}", prayer.event().to_string());
    send_notification(summary, config.urgency());
}

pub fn notify_before_prayer(prayer: &Prayer, duration: chrono::Duration) {
    let summary = format!(
        "Adhan {} in {} minutes",
        prayer.event().to_string(),
        duration.num_minutes()
    );
    send_notification(summary, Urgency::Low);
}
