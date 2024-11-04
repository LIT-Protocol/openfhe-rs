use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// The Plaintext Encodings
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum PlaintextEncodings {
    #[default]
    Invalid = 0,
    CoefPacked,
    Packed,
    String,
    CkksPacked,
}

impl TryFrom<u8> for PlaintextEncodings {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PlaintextEncodings::CoefPacked),
            2 => Ok(PlaintextEncodings::Packed),
            3 => Ok(PlaintextEncodings::String),
            4 => Ok(PlaintextEncodings::CkksPacked),
            _ => Err(Error::InvalidPlaintextEncoding(value as usize)),
        }
    }
}

try_from_int_impl!(
    PlaintextEncodings,
    InvalidPlaintextEncoding,
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
    PlaintextEncodings,
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
enum_serde_impl!(PlaintextEncodings);

impl Display for PlaintextEncodings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlaintextEncodings::Invalid => write!(f, "Invalid"),
            PlaintextEncodings::CoefPacked => write!(f, "CoefPacked"),
            PlaintextEncodings::Packed => write!(f, "Packed"),
            PlaintextEncodings::String => write!(f, "String"),
            PlaintextEncodings::CkksPacked => write!(f, "CkksPacked"),
        }
    }
}

impl FromStr for PlaintextEncodings {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CoefPacked" => Ok(PlaintextEncodings::CoefPacked),
            "Packed" => Ok(PlaintextEncodings::Packed),
            "String" => Ok(PlaintextEncodings::String),
            "CkksPacked" => Ok(PlaintextEncodings::CkksPacked),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing PlaintextEncodings: '{}'",
                s
            ))),
        }
    }
}
