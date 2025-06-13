pub mod generation;

use std::path::PathBuf;

use crate::madhab::Madhab;
use crate::method::Method;
use crate::notification_urgency::NotifUrgency;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

/// Program to notify prayer times
#[derive(Parser)]
#[command(name = "prayer-times")]
// Read from `Cargo.toml`
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Latitude. Defaults to the current location
    #[arg(short = 'l', long)]
    pub latitude: Option<f64>,
    /// Longitude. Defaults to the current location
    #[arg(short = 'L', long)]
    pub longitude: Option<f64>,

    /// Calculation Method to use
    #[arg(short = 'm', long)]
    pub method: Option<Method>,
    /// Madhab to use
    #[arg(short = 'M', long)]
    pub madhab: Option<Madhab>,
    // /// Custom Fajr angle
    // #[arg(long)]
    // pub fajr_angle: Option<f64>,
    // /// Custom Isha angle
    // #[arg(long)]
    // pub isha_angle: Option<f64>,
    /// Minutes to add or remove to the Fajr time
    #[arg(long, allow_hyphen_values = true)]
    pub fajr_mod: Option<i8>,
    /// Minutes to add or remove to the Dohr time
    #[arg(long, allow_hyphen_values = true)]
    pub dohr_mod: Option<i8>,
    /// Minutes to add or remove to the Asr time
    #[arg(long, allow_hyphen_values = true)]
    pub asr_mod: Option<i8>,
    /// Minutes to add or remove to the Maghrib time
    #[arg(long, allow_hyphen_values = true)]
    pub maghrib_mod: Option<i8>,
    /// Minutes to add or remove to the Isha time
    #[arg(long, allow_hyphen_values = true)]
    pub isha_mod: Option<i8>,

    /// Show notification 10 minutes before prayer time [default: false]
    #[arg(long)]
    pub notify_before: Option<bool>,
    /// Custom icon path for notifications
    #[arg(long)]
    pub icon: Option<PathBuf>,

    /// Notification urgency
    #[arg(long)]
    pub urgency: Option<NotifUrgency>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the process that will send notifications on prayers time [default]
    Deamon(DeamonArgs),
    // /// Get the previous prayer
    // Previous(PrayerArgs),
    // /// Get the current prayer
    // Current(PrayerArgs),
    // /// Get the next prayer
    // Next(PrayerArgs),
    /// Get the previous prayer
    Previous,
    /// Get the current prayer
    Current,
    /// Get the next prayer
    Next,
    /// List all the prayers of the current day
    ListPrayers,
    /// List all methods available for the calculation of the prayer times
    ListMethods,
    /// List all madhab available for the calculation of the prayer times
    ListMadhab,
    /// Show the next prayer in a notification to test if everything works
    DryRun,
    /// Get the path of the toml config file
    Config,
    /// Generate shell completions and man pages
    GenerateShell,
}

impl Default for Commands {
    fn default() -> Self {
        // By default, start the deamon
        Self::Deamon(DeamonArgs { interval: None })
    }
}

#[derive(Args)]
pub struct DeamonArgs {
    /// Interval in seconds for checking new prayers
    #[arg(short, long)]
    pub interval: Option<u64>,
}

// #[derive(Args, Clone)]
// pub struct PrayerArgs {
//     #[command(subcommand)]
//     command: Option<PrayerCommands>,
// }
// impl PrayerArgs {
//     pub fn get_command(&self) -> PrayerCommands {
//         self.command.clone().unwrap_or_default()
//     }
// }
//
// #[derive(Subcommand, Default, Clone)]
// pub enum PrayerCommands {
//     /// Get the current prayer
//     #[default]
//     Prayer,
//     /// Get the previous prayer
//     Previous,
//     /// Get the next prayer
//     Next,
// }
