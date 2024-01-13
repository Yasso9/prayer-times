mod notification;
mod prayers;

use chrono::Duration;

use crate::notification::notify;

fn main() {
    println!("Starting Prayer Time Daemon");

    for prayer in prayers::list_prayers() {
        println!(
            "Adhan {} at {}",
            prayer.enum_prayer(),
            prayer.date_time().time()
        );
    }

    let current_prayer = prayers::current();
    println!(
        "Current {} since {}",
        current_prayer.enum_prayer(),
        current_prayer.date_time().time()
    );

    loop {
        let next_prayer = prayers::next();
        let time_remaining = next_prayer.time_remaining();
        println!(
            "{} at {}",
            next_prayer.enum_prayer(),
            next_prayer.date_time().time()
        );
        println!(
            "Time remaining for {} is {:02}:{:02}",
            next_prayer.enum_prayer(),
            time_remaining.num_minutes() / 60,
            time_remaining.num_minutes() % 60
        );

        if time_remaining < Duration::zero() {
            notify(&next_prayer);
        }

        println!("Sleeping...");
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
