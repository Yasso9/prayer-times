use notify_rust::Urgency;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Default, Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum NotifUrgency {
    Low,
    Normal,
    #[default]
    Critical,
}
impl From<NotifUrgency> for Urgency {
    fn from(urgency: NotifUrgency) -> Self {
        match urgency {
            NotifUrgency::Low => Urgency::Low,
            NotifUrgency::Normal => Urgency::Normal,
            NotifUrgency::Critical => Urgency::Critical,
        }
    }
}
