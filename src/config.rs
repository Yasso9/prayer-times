use crate::arguments::Commands;
use crate::madhab::Madhab;
use crate::method::Method;
use crate::notification_urgency::NotifUrgency;
use crate::Arguments;

use notify_rust::Urgency;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize)]
struct PrayerConfig {
    method: Method,
    madhab: Madhab,
    fajr_mod: i8,
    dohr_mod: i8,
    asr_mod: i8,
    maghrib_mod: i8,
    isha_mod: i8,
}
#[derive(Serialize, Deserialize)]
struct NotificationConfig {
    notify_before: bool,
    urgency: NotifUrgency,
    interval: u64,
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    location: Location,
    prayer: PrayerConfig,
    notification: NotificationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: Location { lat: 0., lon: 0. },
            prayer: PrayerConfig {
                method: Method::default(),
                madhab: Madhab::default(),
                fajr_mod: 0,
                dohr_mod: 0,
                asr_mod: 0,
                maghrib_mod: 0,
                isha_mod: 0,
            },
            notification: NotificationConfig {
                notify_before: false,
                urgency: NotifUrgency::Critical,
                interval: 20,
            },
        }
    }
}

fn public_ip() -> Option<String> {
    let mut ip = None;
    // List all of the machine's network interfaces
    for iface in get_if_addrs::get_if_addrs().ok()? {
        if iface.is_loopback() {
            continue;
        }
        let ip_addr = iface.ip().to_string();
        if ip_addr.starts_with("192.168") {
            continue;
        }
        // println!("IP : {:#?}", iface.ip());
        ip = Some(ip_addr);
        // println!("IP : {:#?}", iface.type_id());
        // println!("{:#?}", iface.is_loopback());
    }

    ip
}

fn current_location() -> Option<Location> {
    let info = geolocation::find(public_ip()?.as_str()).ok()?;
    let lat: Result<f64, _> = info.latitude.parse();
    let lon: Result<f64, _> = info.longitude.parse();
    Some(Location {
        lat: lat.ok()?,
        lon: lon.ok()?,
    })
}

impl Config {
    // Generate a new Config from command line arguments
    pub fn new(args: &Arguments) -> Self {
        // println!("{:?}", args);

        let config: Config = confy::load("prayer-times", "config").unwrap_or_default();

        let location: Location;
        if let (Some(latitude), Some(longitude)) = (args.latitude, args.longitude) {
            location = Location {
                lat: latitude,
                lon: longitude,
            };
        } else if config.location.lat != 0. && config.location.lon != 0. {
            location = config.location;
        } else if let Some(auto_location) = current_location() {
            location = auto_location;
        } else {
            panic!("No location provided in config file and impossible to get it automatically");
        }

        let mut interval = config.notification.interval;
        if let Some(command) = &args.command {
            if let Commands::Deamon(deamon) = command {
                if deamon.interval.is_some() {
                    interval = deamon.interval.unwrap();
                }
            }
        }
        if interval == 0 {
            interval = 1;
            println!("Interval cannot be 0, setting it to 1 the minimum value");
        }

        Self {
            location,
            prayer: PrayerConfig {
                method: args.method.clone().unwrap_or(config.prayer.method),
                madhab: args.madhab.clone().unwrap_or(config.prayer.madhab),
                fajr_mod: args.fajr_mod.unwrap_or(config.prayer.fajr_mod),
                dohr_mod: args.dohr_mod.unwrap_or(config.prayer.dohr_mod),
                asr_mod: args.asr_mod.unwrap_or(config.prayer.asr_mod),
                maghrib_mod: args.maghrib_mod.unwrap_or(config.prayer.maghrib_mod),
                isha_mod: args.isha_mod.unwrap_or(config.prayer.isha_mod),
            },
            notification: NotificationConfig {
                notify_before: args
                    .notify_before
                    .unwrap_or(config.notification.notify_before),
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

    pub fn fajr(&self) -> f64 {
        self.prayer.method.fajr_angle() * 60.
    }
    pub fn isha(&self) -> f64 {
        self.prayer.method.isha_angle()
    }
    pub fn shadow_multiplier(&self) -> u8 {
        self.prayer.madhab.shadow_multiplier()
    }

    pub fn fajr_offset(&self) -> f64 {
        self.prayer.fajr_mod as f64 / 60.
    }
    pub fn dhuhr_offset(&self) -> f64 {
        self.prayer.dohr_mod as f64 / 60.
    }
    pub fn asr_offset(&self) -> f64 {
        self.prayer.asr_mod as f64 / 60.
    }
    pub fn maghrib_offset(&self) -> f64 {
        self.prayer.maghrib_mod as f64 / 60.
    }
    pub fn isha_offset(&self) -> f64 {
        self.prayer.isha_mod as f64 / 60.
    }

    pub fn notify_before(&self) -> bool {
        self.notification.notify_before
    }
    pub fn urgency(&self) -> Urgency {
        // TODO : why do I need clone here
        self.notification.urgency.clone().into()
    }
    pub fn interval(&self) -> u64 {
        self.notification.interval
    }
}
