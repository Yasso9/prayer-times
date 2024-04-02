use std::path;

use crate::arguments::Commands;
use crate::event::Event;
use crate::location::current_location;
use crate::location::Location;
use crate::madhab::Madhab;
use crate::method::Method;
use crate::notification_urgency::NotifUrgency;
use crate::Arguments;

use notify_rust::Urgency;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct PrayerConfig {
    method: Method,
    madhab: Madhab,
    fajr_mod: i8,
    shourouk_mod: i8,
    dohr_mod: i8,
    asr_mod: i8,
    maghrib_mod: i8,
    isha_mod: i8,
}
#[derive(Serialize, Deserialize)]
struct NotificationConfig {
    notify_before: bool,
    urgency: NotifUrgency,
    icon: path::PathBuf,
    interval: u64,
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    location: Location,
    prayer: PrayerConfig,
    notification: NotificationConfig,
}

// Get the icon of the notification that should be sent
fn default_icon() -> path::PathBuf {
    let assets_path = if cfg!(debug_assertions) {
        path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
    } else {
        path::PathBuf::from("/usr/share/icons")
    };

    assets_path.join("mosque-svgrepo-com.png")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: Location { lat: 0., lon: 0. },
            prayer: PrayerConfig {
                method: Method::default(),
                madhab: Madhab::default(),
                fajr_mod: 0,
                shourouk_mod: 0,
                dohr_mod: 0,
                asr_mod: 0,
                maghrib_mod: 0,
                isha_mod: 0,
            },
            notification: NotificationConfig {
                notify_before: false,
                urgency: NotifUrgency::Critical,
                icon: default_icon(),
                interval: 20,
            },
        }
    }
}

pub fn config_options<'a>() -> (&'a str, &'a str) {
    const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
    (PROGRAM_NAME, "config")
}

impl Config {
    // Generate a new Config from command line arguments
    pub fn new(args: &Arguments) -> Self {
        // println!("{:?}", args);

        // TODO: get prayer-times from
        let (program, config) = config_options();
        let config: Config = confy::load(program, config).unwrap_or_default();

        let mut is_deamon = false;
        let mut interval = config.notification.interval;
        if let Some(Commands::Deamon(deamon)) = &args.command {
            is_deamon = true;
            if deamon.interval.is_some() {
                interval = deamon.interval.unwrap();
            }
        }
        if interval == 0 {
            interval = 1;
            println!("Interval cannot be 0, setting it to 1 the minimum value");
        }

        let location: Location;
        if let (Some(latitude), Some(longitude)) = (args.latitude, args.longitude) {
            location = Location {
                lat: latitude,
                lon: longitude,
            };
        } else if config.location.lat != 0. && config.location.lon != 0. {
            location = config.location;
        } else if let Some(auto_location) = current_location(is_deamon) {
            location = auto_location;
        } else {
            println!("No location provided in arguments or config file and impossible to get it automatically");
            println!("Run the program using the latitude and longitude arguments or set them in the config file");
            println!("Example : {} --latitude <LAT> --longitude <LON>", program);
            std::process::exit(1);
        }

        Self {
            location,
            prayer: PrayerConfig {
                method: args.method.clone().unwrap_or(config.prayer.method),
                madhab: args.madhab.clone().unwrap_or(config.prayer.madhab),
                fajr_mod: args.fajr_mod.unwrap_or(config.prayer.fajr_mod),
                shourouk_mod: args.shourouk_mod.unwrap_or(config.prayer.shourouk_mod),
                dohr_mod: args.dohr_mod.unwrap_or(config.prayer.dohr_mod),
                asr_mod: args.asr_mod.unwrap_or(config.prayer.asr_mod),
                maghrib_mod: args.maghrib_mod.unwrap_or(config.prayer.maghrib_mod),
                isha_mod: args.isha_mod.unwrap_or(config.prayer.isha_mod),
            },
            notification: NotificationConfig {
                notify_before: args
                    .notify_before
                    .unwrap_or(config.notification.notify_before),
                icon: args.icon.clone().unwrap_or(config.notification.icon),
                urgency: args.urgency.clone().unwrap_or(config.notification.urgency),
                interval,
            },
        }
    }

    pub fn lat(&self) -> f64 {
        self.location.lat
    }
    pub fn lon(&self) -> f64 {
        self.location.lon
    }

    pub fn fajr_angle(&self) -> f64 {
        self.prayer.method.fajr_angle()
    }
    pub fn isha_angle(&self) -> f64 {
        self.prayer.method.isha_angle()
    }
    pub fn shadow_multiplier(&self) -> u8 {
        self.prayer.madhab.shadow_multiplier()
    }

    pub fn offset(&self, event: Event) -> f64 {
        let minutes_mod = match event {
            Event::Fajr => self.prayer.fajr_mod,
            Event::Shourouk => self.prayer.shourouk_mod,
            Event::Dhuhr => self.prayer.dohr_mod,
            Event::Asr => self.prayer.asr_mod,
            Event::Maghrib => self.prayer.maghrib_mod,
            Event::Isha => self.prayer.isha_mod,
        };
        minutes_mod as f64 / 60.
    }

    pub fn notify_before(&self) -> bool {
        self.notification.notify_before
    }
    pub fn urgency(&self) -> Urgency {
        // TODO : why do I need clone here
        self.notification.urgency.clone().into()
    }
    pub fn icon(&self) -> path::PathBuf {
        self.notification.icon.clone()
    }
    pub fn interval(&self) -> u64 {
        self.notification.interval
    }
}
