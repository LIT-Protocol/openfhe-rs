use crate::Error;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// All features supported by public key encryption schemes
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum PkeSchemeFeature {
    /// Public Key Encryption
    Pke = 0x01,
    /// Key Switching
    KeySwitch = 0x02,
    /// Proxy Re-Encryption
    Pre = 0x04,
    LeveledShe = 0x08,
    AdvancedShe = 0x10,
    MultiParty = 0x20,
    Fhe = 0x40,
    SchemeSwitch = 0x80,
}

impl TryFrom<u8> for PkeSchemeFeature {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PkeSchemeFeature::Pke),
            0x02 => Ok(PkeSchemeFeature::KeySwitch),
            0x04 => Ok(PkeSchemeFeature::Pre),
            0x08 => Ok(PkeSchemeFeature::LeveledShe),
            0x10 => Ok(PkeSchemeFeature::AdvancedShe),
            0x20 => Ok(PkeSchemeFeature::MultiParty),
            0x40 => Ok(PkeSchemeFeature::Fhe),
            0x80 => Ok(PkeSchemeFeature::SchemeSwitch),
            _ => Err(Error::InvalidPkeSchemeFeature(value as usize)),
        }
    }
}

try_from_int_impl!(
    PkeSchemeFeature,
    InvalidPkeSchemeFeature,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    usize,
    isize
);
into_int_impl!(
    PkeSchemeFeature,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    usize,
    isize
);
enum_serde_impl!(PkeSchemeFeature);

impl Display for PkeSchemeFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pke => write!(f, "Pke"),
            Self::KeySwitch => write!(f, "KeySwitch"),
            Self::Pre => write!(f, "Pre"),
            Self::LeveledShe => write!(f, "LeveledShe"),
            Self::AdvancedShe => write!(f, "AdvancedShe"),
            Self::MultiParty => write!(f, "Multiparty"),
            Self::Fhe => write!(f, "Fhe"),
            Self::SchemeSwitch => write!(f, "SchemeSwitch"),
        }
    }
}

impl FromStr for PkeSchemeFeature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pke" => Ok(PkeSchemeFeature::Pke),
            "KeySwitch" => Ok(PkeSchemeFeature::KeySwitch),
            "Pre" => Ok(PkeSchemeFeature::Pre),
            "LeveledShe" => Ok(PkeSchemeFeature::LeveledShe),
            "AdvancedShe" => Ok(PkeSchemeFeature::AdvancedShe),
            "Multiparty" => Ok(PkeSchemeFeature::MultiParty),
            "Fhe" => Ok(PkeSchemeFeature::Fhe),
            "SchemeSwitch" => Ok(PkeSchemeFeature::SchemeSwitch),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing PkeSchemeFeature: '{}'",
                s
            ))),
        }
    }
}
