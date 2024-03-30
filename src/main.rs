mod arguments;
mod calculations;
mod config;
mod event;
mod location;
mod madhab;
mod method;
mod notification;
mod notification_urgency;
mod prayer;
mod prayers;

use self::{
    arguments::Arguments,
    arguments::Commands,
    config::Config,
    madhab::Madhab,
    method::Method,
    notification::{notify_before_prayer, notify_prayer},
};

use clap::Parser;

fn background_process(config: &Config) {
    let mut next_prayer = prayers::next(config);
    let mut is_notified_before = false;

    println!("Starting Prayer Time Daemon");
    println!("Waiting for next prayer...");
    loop {
        println!("{}", next_prayer.text_duration());
        println!("{}", next_prayer.text_time());

        if next_prayer.time_has_passed() {
            println!("Prayer time has passed");
            // Notification only if it's the current prayer.
            // If it's not the current prayer, it means that the system have been suspended
            // so we are currently in an other prayer
            if next_prayer == prayers::current(config) {
                notify_prayer(&next_prayer, config);
            }

            // Update next prayer
            next_prayer = prayers::next(config);
            is_notified_before = false;
        } else if config.notify_before()
            && !is_notified_before
            && next_prayer.time_remaining() < chrono::Duration::minutes(11)
        {
            notify_before_prayer(&next_prayer, next_prayer.time_remaining(), config);
            is_notified_before = true;
        }

        std::thread::sleep(std::time::Duration::from_secs(config.interval()));
    }
}

fn main() {
    let args = Arguments::parse();
    let config = Config::new(&args);

    match args.command.unwrap_or_default() {
        Commands::Deamon(_deamon) => {
            background_process(&config);
        }
        Commands::Current => {
            let prayer = prayers::current(&config);
            println!("{}", prayer.text_time());
        }
        Commands::Next => {
            let prayer = prayers::next(&config);
            println!("{}", prayer.text_time());
        }
        Commands::ListPrayers => {
            println!("Prayer times:");
            for prayer in prayers::list_prayers(&config) {
                println!("{}", prayer.text_time());
            }
        }
        Commands::ListMethods => {
            println!("Methods:");
            Method::list_all();
        }
        Commands::ListMadhab => {
            println!("Madhab:");
            Madhab::list_all();
        }
        Commands::DryRun => {
            let next_prayer = prayers::next(&config);
            notify_prayer(&next_prayer, &config);
        }
        Commands::Config => {
            let (program, config) = config::config_options();
            let result = confy::get_configuration_file_path(program, config);
            match result {
                Ok(path) => {
                    println!("Config file: {}", path.display());
                }
                Err(e) => {
                    println!("Error reading config file: {}", e);
                }
            }
        }
    }
}
