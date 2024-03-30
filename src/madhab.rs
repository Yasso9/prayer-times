use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, Serialize, Deserialize, EnumIter, Display)]
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

    pub fn list_all() {
        for variant in Self::iter() {
            println!("{}", variant);
        }
    }
}
impl Default for Madhab {
    fn default() -> Self {
        Madhab::Shafi
    }
}
