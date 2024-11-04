use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Execution mode for the FHE scheme.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum ExecutionMode {
    Evaluation = 0,
    NoiseEstimation,
}

impl TryFrom<u8> for ExecutionMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExecutionMode::Evaluation),
            1 => Ok(ExecutionMode::NoiseEstimation),
            _ => Err(Error::InvalidExecutionMode(value as usize)),
        }
    }
}

try_from_int_impl!(
    ExecutionMode,
    InvalidExecutionMode,
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
    ExecutionMode,
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
enum_serde_impl!(ExecutionMode);

impl Display for ExecutionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionMode::Evaluation => write!(f, "Evaluation"),
            ExecutionMode::NoiseEstimation => write!(f, "NoiseEstimation"),
        }
    }
}

impl FromStr for ExecutionMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Evaluation" => Ok(ExecutionMode::Evaluation),
            "NoiseEstimation" => Ok(ExecutionMode::NoiseEstimation),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing ExecutionMode: '{}'",
                s
            ))),
        }
    }
}
