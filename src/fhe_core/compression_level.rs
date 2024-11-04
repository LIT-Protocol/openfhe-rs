use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Defining the level to which the input ciphertext is brought to before
/// interactive multi-party bootstrapping
#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum CompressionLevel {
    // we don't support 0 or 1 compression levels
    // do not change values here
    Compact = 2, // more efficient with stronger security assumption
    Slack = 3,   // less efficient with weaker security assumption
}

impl TryFrom<u8> for CompressionLevel {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(CompressionLevel::Compact),
            3 => Ok(CompressionLevel::Slack),
            _ => Err(Error::InvalidCompressionLevel(value as usize)),
        }
    }
}

try_from_int_impl!(
    CompressionLevel,
    InvalidCompressionLevel,
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
    CompressionLevel,
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
enum_serde_impl!(CompressionLevel);

impl Display for CompressionLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompressionLevel::Compact => write!(f, "Compact"),
            CompressionLevel::Slack => write!(f, "Slack"),
        }
    }
}

impl FromStr for CompressionLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Compact" => Ok(CompressionLevel::Compact),
            "Slack" => Ok(CompressionLevel::Slack),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing CompressionLevel: '{}'",
                s
            ))),
        }
    }
}
