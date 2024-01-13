use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

use crate::prayers::Prayer;

// Get the icon of the notification that should be sent
fn get_icon() -> Result<PathBuf, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.join("assets").join("mosque-svgrepo-com.png"))
}

// Send notification to the screen
pub fn notify(prayer: &Prayer) {
    let notification = Notification::new()
        .summary(&format!(
            "Adhan {}",
            prayer.enum_prayer().to_string().as_str()
        ))
        .icon(get_icon().unwrap_or_default().to_str().unwrap_or_default())
        .urgency(Urgency::Critical)
        .show();
    match notification {
        Ok(_) => println!("Notification sent"),
        Err(_) => println!("Failed to send notification"),
    }
}
