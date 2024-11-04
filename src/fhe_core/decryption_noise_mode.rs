use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Decryption noise mode
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum DecryptionNoiseMode {
    #[default]
    FixedNoiseDecrypt = 0,
    NoiseFloodingDecrypt,
}

impl TryFrom<u8> for DecryptionNoiseMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DecryptionNoiseMode::FixedNoiseDecrypt),
            1 => Ok(DecryptionNoiseMode::NoiseFloodingDecrypt),
            _ => Err(Error::InvalidDecryptionNoiseMode(value as usize)),
        }
    }
}

try_from_int_impl!(
    DecryptionNoiseMode,
    InvalidDecryptionNoiseMode,
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
    DecryptionNoiseMode,
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
enum_serde_impl!(DecryptionNoiseMode);

impl Display for DecryptionNoiseMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DecryptionNoiseMode::FixedNoiseDecrypt => write!(f, "FixedNoiseDecrypt"),
            DecryptionNoiseMode::NoiseFloodingDecrypt => write!(f, "NoiseFloodingDecrypt"),
        }
    }
}

impl FromStr for DecryptionNoiseMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FixedNoiseDecrypt" => Ok(DecryptionNoiseMode::FixedNoiseDecrypt),
            "NoiseFloodingDecrypt" => Ok(DecryptionNoiseMode::NoiseFloodingDecrypt),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing DecryptionNoiseMode: '{}'",
                s
            ))),
        }
    }
}
