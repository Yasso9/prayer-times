use notify_rust::Urgency;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum NotifUrgency {
    Low,
    Normal,
    Critical,
}
impl Default for NotifUrgency {
    fn default() -> Self {
        NotifUrgency::Critical
    }
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
