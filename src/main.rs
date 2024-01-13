mod notification;
mod prayers;

use crate::notification::notify;

fn main() {
    println!("Starting Prayer Time Daemon");

    const SLEEPING_TIME: std::time::Duration = std::time::Duration::from_secs(30);

    for prayer in prayers::list_prayers() {
        println!("Adhan {} at {}", prayer.enum_prayer(), prayer.date_time());
    }

    let mut prayer = prayers::current();
    println!("Current {}", prayer.date_time());
    println!("Current {}", prayer.enum_prayer());

    let next = prayers::next();
    println!("Next {}", next.date_time());
    println!("Next {}", next.enum_prayer());

    loop {
        let new_prayer = prayers::current();

        if prayer.date_time() != new_prayer.date_time() {
            notify(&new_prayer);
            prayer = new_prayer;
        }

        println!("Sleeping...");
        std::thread::sleep(SLEEPING_TIME);
    }
}
