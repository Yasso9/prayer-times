mod calculations;
mod config;
mod event;
mod notification;
mod prayer;
mod prayers;

use self::{
    config::Config,
    notification::{notify_before_prayer, notify_prayer},
};

fn main() {
    println!("Starting Prayer Time Daemon");
    println!("Waiting for next prayer...");

    let config = Config::new();

    println!("\nPrayer times:");
    for prayer in prayers::list_prayers(&config) {
        println!("{}", prayer.text_time());
    }

    let mut next_prayer = prayers::next(&config);

    // For test purpose
    if cfg!(debug_assertions) {
        notify_prayer(&next_prayer);
    }

    let mut is_notified_before = false;
    loop {
        println!("{}", next_prayer.text_duration());
        println!("{}", next_prayer.text_time());

        if next_prayer.time_has_passed() {
            println!("Prayer time has passed");
            // Notification only if it's the current prayer.
            // If it's not the current prayer, it means that the system have been suspended
            // so we Are currently in an other prayer
            if next_prayer == prayers::current(&config) {
                notify_prayer(&next_prayer);
            }

            // Update next prayer
            next_prayer = prayers::next(&config);
            is_notified_before = false;
        } else if config.notify_before()
            && !is_notified_before
            && next_prayer.time_remaining() < chrono::Duration::minutes(11)
        {
            notify_before_prayer(&next_prayer, next_prayer.time_remaining());
            is_notified_before = true;
        }

        println!("Sleeping...");
        std::thread::sleep(std::time::Duration::from_secs(20));
    }
}
