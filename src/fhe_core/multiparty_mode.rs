use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum MultipartyMode {
    #[default]
    InvalidMultipartyMode = 0,
    FixedNoiseMultiparty,
    NoiseFloodingMultiparty,
}

impl TryFrom<u8> for MultipartyMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MultipartyMode::FixedNoiseMultiparty),
            2 => Ok(MultipartyMode::NoiseFloodingMultiparty),
            _ => Err(Error::InvalidMultipartyMode(value as usize)),
        }
    }
}

try_from_int_impl!(
    MultipartyMode,
    InvalidMultipartyMode,
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
    MultipartyMode,
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
enum_serde_impl!(MultipartyMode);

impl Display for MultipartyMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MultipartyMode::FixedNoiseMultiparty => write!(f, "Fixed Noise Multiparty"),
            MultipartyMode::NoiseFloodingMultiparty => write!(f, "Noise Flooding Multiparty"),
            _ => write!(f, "Invalid Multiparty Mode"),
        }
    }
}

impl FromStr for MultipartyMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fixed Noise Multiparty" => Ok(MultipartyMode::FixedNoiseMultiparty),
            "Noise Flooding Multiparty" => Ok(MultipartyMode::NoiseFloodingMultiparty),
            _ => Err(Error::ParseError(format!(
                "Invalid multiparty mode: '{}'",
                s
            ))),
        }
    }
}
