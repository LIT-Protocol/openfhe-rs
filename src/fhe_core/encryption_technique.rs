use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Encryption technique used in FHE operations
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum EncryptionTechnique {
    #[default]
    Standard = 0,
    Extended,
}

impl TryFrom<u8> for EncryptionTechnique {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EncryptionTechnique::Standard),
            1 => Ok(EncryptionTechnique::Extended),
            _ => Err(Error::InvalidEncryptionTechnique(value as usize)),
        }
    }
}

try_from_int_impl!(
    EncryptionTechnique,
    InvalidEncryptionTechnique,
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
    EncryptionTechnique,
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
enum_serde_impl!(EncryptionTechnique);

impl Display for EncryptionTechnique {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionTechnique::Standard => write!(f, "Standard"),
            EncryptionTechnique::Extended => write!(f, "Extended"),
        }
    }
}

impl FromStr for EncryptionTechnique {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(EncryptionTechnique::Standard),
            "Extended" => Ok(EncryptionTechnique::Extended),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing EncryptionTechnique: '{}'",
                s
            ))),
        }
    }
}
