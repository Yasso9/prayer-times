use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum Madhab {
    Shafi,
    Hanafi,
}
impl Madhab {
    pub fn shadow_multiplier(&self) -> u8 {
        match self {
            Madhab::Shafi => 1,
            Madhab::Hanafi => 2,
        }
    }
}
impl Default for Madhab {
    fn default() -> Self {
        Madhab::Shafi
    }
}
