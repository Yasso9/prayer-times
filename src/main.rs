mod notification;
mod prayers;

use chrono::Duration;

use crate::{
    notification::{notify_before_prayer, notify_prayer},
    prayers::arguments::Config,
};

fn main() {
    println!("Starting Prayer Time Daemon");

    // Informations
    for prayer in prayers::list_prayers() {
        println!("Adhan {} at {}", prayer.event(), prayer.date_time().time());
    }
    let current_prayer = prayers::current();
    println!(
        "Current {} since {}",
        current_prayer.event(),
        current_prayer.date_time().time()
    );

    let mut next_prayer = prayers::next();
    println!(
        "{} at {}",
        next_prayer.event(),
        next_prayer.date_time().time()
    );

    let config = Config::new();

    loop {
        let time_remaining = next_prayer.time_remaining();
        println!(
            "Time remaining for {} is {:02}:{:02}",
            next_prayer.event(),
            time_remaining.num_minutes() / 60,
            time_remaining.num_minutes() % 60
        );

        if config.notify_before() && time_remaining < Duration::minutes(10) {
            notify_before_prayer(&next_prayer, time_remaining);
        }

        if time_remaining < Duration::zero() {
            notify_prayer(&next_prayer);
            // Update next prayer
            next_prayer = prayers::next();
        }

        println!("Sleeping...");
        std::thread::sleep(std::time::Duration::from_secs(20));
    }
}
