use crate::madhab::Madhab;
use crate::method::Method;
use crate::notification_urgency::NotifUrgency;
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

    /// Minutes to add or remove to the Fajr time
    #[arg(long)]
    pub fajr_mod: Option<i8>,
    /// Minutes to add or remove to the Dohr time
    #[arg(long)]
    pub dohr_mod: Option<i8>,
    /// Minutes to add or remove to the Asr time
    #[arg(long)]
    pub asr_mod: Option<i8>,
    /// Minutes to add or remove to the Maghrib time
    #[arg(long)]
    pub maghrib_mod: Option<i8>,
    /// Minutes to add or remove to the Isha time
    #[arg(long)]
    pub isha_mod: Option<i8>,

    /// Show notification 10 minutes before prayer time [default: false]
    #[arg(long)]
    pub notify_before: Option<bool>,

    /// Notification urgency
    #[arg(long)]
    pub urgency: Option<NotifUrgency>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the deamon that will send notifications on prayers time [default]
    Deamon,
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
    DryRunNotification,
}
// give default implementation
impl Default for Commands {
    fn default() -> Self {
        Self::Deamon
    }
}
