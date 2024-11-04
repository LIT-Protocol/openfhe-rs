use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Proxy re-encryption mode
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum ProxyReEncryptionMode {
    #[default]
    NotSet = 0,
    IndCpa,
    FixedNoiseHra,
    NoiseFloodingHra,
}

impl TryFrom<u8> for ProxyReEncryptionMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotSet),
            1 => Ok(Self::IndCpa),
            2 => Ok(Self::FixedNoiseHra),
            3 => Ok(Self::NoiseFloodingHra),
            _ => Err(Error::InvalidProxyReEncryptionMode(value as usize)),
        }
    }
}

try_from_int_impl!(
    ProxyReEncryptionMode,
    InvalidProxyReEncryptionMode,
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
    ProxyReEncryptionMode,
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
enum_serde_impl!(ProxyReEncryptionMode);

impl Display for ProxyReEncryptionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSet => write!(f, "NotSet"),
            Self::IndCpa => write!(f, "IndCpa"),
            Self::FixedNoiseHra => write!(f, "FixedNoiseHra"),
            Self::NoiseFloodingHra => write!(f, "NoiseFloodingHra"),
        }
    }
}

impl FromStr for ProxyReEncryptionMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotSet" => Ok(Self::NotSet),
            "IndCpa" => Ok(Self::IndCpa),
            "FixedNoiseHra" => Ok(Self::FixedNoiseHra),
            "NoiseFloodingHra" => Ok(Self::NoiseFloodingHra),
            _ => Err(Error::ParseError(format!("invalid string when parsing ProxyReEncryptionMode: '{}'. Expected 'NotSet', 'IndCpa', 'FixedNoiseHra' or 'NoiseFloodingHra'", s)))
        }
    }
}
