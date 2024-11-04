use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Key Switch Technique
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum KeySwitchTechnique {
    #[default]
    InvalidKsTech = 0,
    Bv,
    Hybrid,
}

impl TryFrom<u8> for KeySwitchTechnique {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KeySwitchTechnique::Bv),
            2 => Ok(KeySwitchTechnique::Hybrid),
            _ => Err(Error::InvalidKeySwitchTechnique(value as usize)),
        }
    }
}

try_from_int_impl!(
    KeySwitchTechnique,
    InvalidKeySwitchTechnique,
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
    KeySwitchTechnique,
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
enum_serde_impl!(KeySwitchTechnique);

impl Display for KeySwitchTechnique {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KeySwitchTechnique::InvalidKsTech => write!(f, "Invalid Key Switch Technique"),
            KeySwitchTechnique::Bv => write!(f, "Bv"),
            KeySwitchTechnique::Hybrid => write!(f, "Hybrid"),
        }
    }
}

impl FromStr for KeySwitchTechnique {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bv" => Ok(KeySwitchTechnique::Bv),
            "Hybrid" => Ok(KeySwitchTechnique::Hybrid),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing KeySwitchTechnique: '{}'",
                s
            ))),
        }
    }
}
