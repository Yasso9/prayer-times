use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};

pub fn old_prayer_deamon() {
    println!("Starting Old Prayer Time Daemon");

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
    send_notification("Asr");

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
