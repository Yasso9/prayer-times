use crate::config::Config;
use crate::prayer::Prayer;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

fn send_notification(summary: String, urgency: Urgency, icon: PathBuf) {
    let mut notification = Notification::new(); // so the notification will live
    let notification = notification.summary(&summary).urgency(urgency);

    // ne pas faire un expect mais un simple if suffit
    let full_path = std::fs::canonicalize(icon).expect("Failed to resolve icon path");
    if let Some(icon_str) = full_path.to_str() {
        println!("Setting icon for notification: {}", icon_str);
        notification.icon(icon_str);
    } else {
        println!("Failed to set icon for notification");
    }

    match notification.show() {
        Ok(_) => println!("Notification sent"),
        Err(_) => println!("Failed to send notification"),
    }
}

// TODO on a pas besoin de Prayer mais juste du string
pub fn notify_prayer(prayer: &Prayer, config: &Config) {
    let summary = format!("Adhan {}", prayer.event());
    send_notification(summary, config.urgency(), config.icon());
}

pub fn notify_before_prayer(prayer: &Prayer, duration: chrono::Duration, config: &Config) {
    let summary = format!(
        "Adhan {} in {} minutes",
        prayer.event(),
        duration.num_minutes()
    );
    send_notification(summary, Urgency::Low, config.icon());
}
