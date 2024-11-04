use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// The Hash Algorithm used for hashing the data
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum HashAlgorithm {
    /// SHA-256
    #[default]
    Sha256 = 0,
    /// SHA-512
    Sha512 = 1,
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sha256),
            1 => Ok(Self::Sha512),
            _ => Err(Error::InvalidHashAlgorithm(value as usize)),
        }
    }
}

try_from_int_impl!(
    HashAlgorithm,
    InvalidHashAlgorithm,
    i8,
    u16,
    i16,
    i32,
    u32,
    i64,
    u64,
    usize,
    isize
);
into_int_impl!(
    HashAlgorithm,
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
enum_serde_impl!(HashAlgorithm);

impl Display for HashAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "Sha256"),
            Self::Sha512 => write!(f, "Sha512"),
        }
    }
}

impl FromStr for HashAlgorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sha256" => Ok(Self::Sha256),
            "Sha512" => Ok(Self::Sha512),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing HashAlgorithm: '{}'. Expected 'Sha256' or 'Sha512'",
                s
            ))),
        }
    }
}
