use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum PolynomialFormat {
    /// Ring is evaluation representation
    #[default]
    Evaluation = 0,
    /// Ring is coefficient representation
    Coefficient = 1,
}

impl TryFrom<u8> for PolynomialFormat {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Evaluation),
            1 => Ok(Self::Coefficient),
            _ => Err(Error::InvalidPolynomialFormat(value as usize)),
        }
    }
}

try_from_int_impl!(
    PolynomialFormat,
    InvalidPolynomialFormat,
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
    PolynomialFormat,
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
enum_serde_impl!(PolynomialFormat);

impl Display for PolynomialFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluation => write!(f, "Evaluation"),
            Self::Coefficient => write!(f, "Coefficient"),
        }
    }
}

impl FromStr for PolynomialFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Evaluation" => Ok(Self::Evaluation),
            "Coefficient" => Ok(Self::Coefficient),
            _ => Err(Error::ParseError(format!("invalid string when parsing PolynomialFormat: '{}'. Expected 'Evaluation' or 'Coefficient'", s)))
        }
    }
}
