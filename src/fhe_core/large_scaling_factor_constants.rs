use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Constants for large scaling factor
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum LargeScalingFactorConstants {
    MaxBitsInWord = 61,
    MaxLogStep = 60,
}

impl TryFrom<u8> for LargeScalingFactorConstants {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            61 => Ok(LargeScalingFactorConstants::MaxBitsInWord),
            60 => Ok(LargeScalingFactorConstants::MaxLogStep),
            _ => Err(Error::InvalidLargeScalingFactorConstant(value as usize)),
        }
    }
}

try_from_int_impl!(
    LargeScalingFactorConstants,
    InvalidLargeScalingFactorConstant,
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
    LargeScalingFactorConstants,
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
enum_serde_impl!(LargeScalingFactorConstants);

impl Display for LargeScalingFactorConstants {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LargeScalingFactorConstants::MaxBitsInWord => write!(f, "MaxBitsInWord"),
            LargeScalingFactorConstants::MaxLogStep => write!(f, "MaxLogStep"),
        }
    }
}

impl FromStr for LargeScalingFactorConstants {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MaxBitsInWord" => Ok(LargeScalingFactorConstants::MaxBitsInWord),
            "MaxLogStep" => Ok(LargeScalingFactorConstants::MaxLogStep),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing LargeScalingFactorConstants: '{}'",
                s
            ))),
        }
    }
}
