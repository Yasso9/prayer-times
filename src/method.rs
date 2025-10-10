use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Default, Debug, Clone, EnumString, Serialize, Deserialize, EnumIter, Display)]
pub enum Method {
    #[default]
    MuslimWorldLeague,
    NorthAmerica,
    Egyptian,
    UmmAlQura,
    Karachi,
    Tehran,
    Jafari,
    FranceUOIF,
    FranceGMP,
    // Gulf,
    // Kuwait,
    // Qatar,
    // Singapore,
    // France18,
    // Turkey,
    // Russia,
    // Tunisia,
    // Algeria,
    // Indonesia,
    // Morocco,
    // Lisbon,
}
impl Method {
    pub fn fajr_angle(&self) -> f64 {
        match self {
            Method::MuslimWorldLeague => 18.,
            Method::NorthAmerica => 15.,
            Method::Egyptian => 19.5,
            Method::UmmAlQura => 18.5,
            Method::Karachi => 18.,
            Method::Tehran => 17.7,
            Method::Jafari => 16.,
            Method::FranceUOIF => 12.,
            Method::FranceGMP => 18.,
        }
    }
    pub fn isha_angle(&self) -> f64 {
        match self {
            Method::MuslimWorldLeague => 17.,
            Method::NorthAmerica => 15.,
            Method::Egyptian => 17.5,
            Method::UmmAlQura => 18.5, // Wrong
            Method::Karachi => 18.,
            Method::Tehran => 14.,
            Method::Jafari => 14.,
            Method::FranceUOIF => 12.,
            Method::FranceGMP => 18.,
        }
    }

    pub fn list_all() {
        for variant in Method::iter() {
            println!(
                "{} : [ fajr angle: {}, isha angle: {} ]",
                variant,
                variant.fajr_angle(),
                variant.isha_angle()
            );
        }
    }
}
