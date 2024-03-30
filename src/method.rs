use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum Method {
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
    // Lisboa,
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
}
impl Default for Method {
    fn default() -> Self {
        Method::MuslimWorldLeague
    }
}
