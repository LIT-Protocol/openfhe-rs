use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Multiplication technique used in the FHE scheme.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum MultiplicationTechnique {
    #[default]
    Behz = 0,
    Hps,
    HpsPoverq,
    HpsPoverqLeveled,
}

impl TryFrom<u8> for MultiplicationTechnique {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MultiplicationTechnique::Behz),
            1 => Ok(MultiplicationTechnique::Hps),
            2 => Ok(MultiplicationTechnique::HpsPoverq),
            3 => Ok(MultiplicationTechnique::HpsPoverqLeveled),
            _ => Err(Error::InvalidMultiplicationTechnique(value as usize)),
        }
    }
}

try_from_int_impl!(
    MultiplicationTechnique,
    InvalidMultiplicationTechnique,
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
    MultiplicationTechnique,
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
enum_serde_impl!(MultiplicationTechnique);

impl Display for MultiplicationTechnique {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MultiplicationTechnique::Behz => write!(f, "Behz"),
            MultiplicationTechnique::Hps => write!(f, "Hps"),
            MultiplicationTechnique::HpsPoverq => write!(f, "HpsPoverq"),
            MultiplicationTechnique::HpsPoverqLeveled => write!(f, "HpsPoverqLeveled"),
        }
    }
}

impl FromStr for MultiplicationTechnique {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Behz" => Ok(MultiplicationTechnique::Behz),
            "Hps" => Ok(MultiplicationTechnique::Hps),
            "HpsPoverq" => Ok(MultiplicationTechnique::HpsPoverq),
            "HpsPoverqLeveled" => Ok(MultiplicationTechnique::HpsPoverqLeveled),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing MultiplicationTechnique: '{}'",
                s
            ))),
        }
    }
}
