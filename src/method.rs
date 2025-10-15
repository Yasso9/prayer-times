use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamValue {
    /// numeric angle in degrees (e.g. 18, 19.5)
    Angle(f64),
    /// minutes after maghrib / minutes for Isha (e.g. 90)
    Minutes(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub fajr: ParamValue,
    pub isha: ParamValue,
}

#[derive(Debug, Clone, Copy)]
pub struct Method {
    pub name: &'static str,
    pub params: Params,
}

// https://api.aladhan.com/v1/methods

pub const MWL: Method = Method {
    name: "Muslim World League",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(17.0),
    },
};
pub const ISNA: Method = Method {
    name: "Islamic Society of North America (ISNA)",
    params: Params {
        fajr: ParamValue::Angle(15.0),
        isha: ParamValue::Angle(15.0),
    },
};

pub const EGYPT: Method = Method {
    name: "Egyptian General Authority of Survey",
    params: Params {
        fajr: ParamValue::Angle(19.5),
        isha: ParamValue::Angle(17.5),
    },
};

pub const MAKKAH: Method = Method {
    name: "Umm Al-Qura University, Makkah",
    params: Params {
        fajr: ParamValue::Angle(18.5),
        isha: ParamValue::Minutes(90),
    },
};

pub const KARACHI: Method = Method {
    name: "University of Islamic Sciences, Karachi",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(18.0),
    },
};

pub const TEHRAN: Method = Method {
    name: "Institute of Geophysics, University of Tehran",
    params: Params {
        fajr: ParamValue::Angle(17.7),
        isha: ParamValue::Angle(14.0),
    },
};

pub const JAFARI: Method = Method {
    name: "Shia Ithna-Ashari, Leva Institute, Qum",
    params: Params {
        fajr: ParamValue::Angle(16.0),
        isha: ParamValue::Angle(14.0),
    },
};

pub const GULF: Method = Method {
    name: "Gulf Region",
    params: Params {
        fajr: ParamValue::Angle(19.5),
        isha: ParamValue::Minutes(90),
    },
};

pub const KUWAIT: Method = Method {
    name: "Kuwait",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(17.5),
    },
};

pub const QATAR: Method = Method {
    name: "Qatar",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Minutes(90),
    },
};

pub const SINGAPORE: Method = Method {
    name: "Majlis Ugama Islam Singapura, Singapore",
    params: Params {
        fajr: ParamValue::Angle(20.0),
        isha: ParamValue::Angle(18.0),
    },
};

pub const FRANCE: Method = Method {
    name: "Union Organization Islamic de France",
    params: Params {
        fajr: ParamValue::Angle(12.0),
        isha: ParamValue::Angle(12.0),
    },
};

pub const TURKEY: Method = Method {
    name: "Diyanet İşleri Başkanlığı, Turkey",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(17.0),
    },
};

pub const RUSSIA: Method = Method {
    name: "Spiritual Administration of Muslims of Russia",
    params: Params {
        fajr: ParamValue::Angle(16.0),
        isha: ParamValue::Angle(15.0),
    },
};

pub const DUBAI: Method = Method {
    name: "Dubai",
    params: Params {
        fajr: ParamValue::Angle(18.2),
        isha: ParamValue::Angle(18.2),
    },
};

pub const JAKIM: Method = Method {
    name: "Jabatan Kemajuan Islam Malaysia (JAKIM)",
    params: Params {
        fajr: ParamValue::Angle(20.0),
        isha: ParamValue::Angle(18.0),
    },
};

pub const TUNISIA: Method = Method {
    name: "Tunisia",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(18.0),
    },
};

pub const ALGERIA: Method = Method {
    name: "Algeria",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(17.0),
    },
};

pub const KEMENAG: Method = Method {
    name: "Kementerian Agama Republik Indonesia",
    params: Params {
        fajr: ParamValue::Angle(20.0),
        isha: ParamValue::Angle(18.0),
    },
};

pub const MOROCCO: Method = Method {
    name: "Morocco",
    params: Params {
        fajr: ParamValue::Angle(19.0),
        isha: ParamValue::Angle(17.0),
    },
};

pub const PORTUGAL: Method = Method {
    name: "Comunidade Islamica de Lisboa",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Minutes(77),
    },
};

pub const JORDAN: Method = Method {
    name: "Ministry of Awqaf, Islamic Affairs and Holy Places, Jordan",
    params: Params {
        fajr: ParamValue::Angle(18.0),
        isha: ParamValue::Angle(18.0),
    },
};

#[derive(
    Default, Debug, Clone, Copy, PartialEq, EnumString, Serialize, Deserialize, EnumIter, Display,
)]
pub enum MethodVariant {
    #[default]
    MWL,
    ISNA,
    EGYPT,
    MAKKAH,
    KARACHI,
    TEHRAN,
    JAFARI,
    GULF,
    KUWAIT,
    QATAR,
    SINGAPORE,
    FRANCE,
    TURKEY,
    RUSSIA,
    DUBAI,
    JAKIM,
    TUNISIA,
    ALGERIA,
    KEMENAG,
    MOROCCO,
    PORTUGAL,
    JORDAN,
}

impl MethodVariant {
    /// Get the method data for this variant
    pub fn get(&self) -> &'static Method {
        match self {
            MethodVariant::MWL => &MWL,
            MethodVariant::ISNA => &ISNA,
            MethodVariant::EGYPT => &EGYPT,
            MethodVariant::MAKKAH => &MAKKAH,
            MethodVariant::KARACHI => &KARACHI,
            MethodVariant::TEHRAN => &TEHRAN,
            MethodVariant::JAFARI => &JAFARI,
            MethodVariant::GULF => &GULF,
            MethodVariant::KUWAIT => &KUWAIT,
            MethodVariant::QATAR => &QATAR,
            MethodVariant::SINGAPORE => &SINGAPORE,
            MethodVariant::FRANCE => &FRANCE,
            MethodVariant::TURKEY => &TURKEY,
            MethodVariant::RUSSIA => &RUSSIA,
            MethodVariant::DUBAI => &DUBAI,
            MethodVariant::JAKIM => &JAKIM,
            MethodVariant::TUNISIA => &TUNISIA,
            MethodVariant::ALGERIA => &ALGERIA,
            MethodVariant::KEMENAG => &KEMENAG,
            MethodVariant::MOROCCO => &MOROCCO,
            MethodVariant::PORTUGAL => &PORTUGAL,
            MethodVariant::JORDAN => &JORDAN,
        }
    }

    pub fn list() {
        for variant in MethodVariant::iter() {
            let method = variant.get();
            let fajr_str = match method.params.fajr {
                ParamValue::Angle(a) => format!("{}°", a),
                ParamValue::Minutes(m) => format!("{} min", m),
            };
            let isha_str = match method.params.isha {
                ParamValue::Angle(a) => format!("{}°", a),
                ParamValue::Minutes(m) => format!("{} min", m),
            };
            println!(
                "{} : [ fajr: {}, isha: {} ]",
                method.name, fajr_str, isha_str
            );
        }
    }
}
