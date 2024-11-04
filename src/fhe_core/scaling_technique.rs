use crate::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// The scaling technique
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[repr(u8)]
pub enum ScalingTechnique {
    #[default]
    FixedManual = 0,
    FixedAuto,
    FlexibleAuto,
    FlexibleAutoExt,
    NoRescale,
    InvalidRsTechnique,
}

impl TryFrom<u8> for ScalingTechnique {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScalingTechnique::FixedManual),
            1 => Ok(ScalingTechnique::FixedAuto),
            2 => Ok(ScalingTechnique::FlexibleAuto),
            3 => Ok(ScalingTechnique::FlexibleAutoExt),
            4 => Ok(ScalingTechnique::NoRescale),
            _ => Err(Error::InvalidScalingTechnique(value as usize)),
        }
    }
}

try_from_int_impl!(
    ScalingTechnique,
    InvalidScalingTechnique,
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
    ScalingTechnique,
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
enum_serde_impl!(ScalingTechnique);

impl Display for ScalingTechnique {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FixedManual => write!(f, "FixedManual"),
            Self::FixedAuto => write!(f, "FixedAuto"),
            Self::FlexibleAuto => write!(f, "FlexibleAuto"),
            Self::FlexibleAutoExt => write!(f, "FlexibleAutoExt"),
            Self::NoRescale => write!(f, "NoRescale"),
            Self::InvalidRsTechnique => write!(f, "InvalidRsTechnique"),
        }
    }
}

impl FromStr for ScalingTechnique {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FixedManual" => Ok(ScalingTechnique::FixedManual),
            "FixedAuto" => Ok(ScalingTechnique::FixedAuto),
            "FlexibleAuto" => Ok(ScalingTechnique::FlexibleAuto),
            "FlexibleAutoExt" => Ok(ScalingTechnique::FlexibleAutoExt),
            "NoRescale" => Ok(ScalingTechnique::NoRescale),
            _ => Err(Error::ParseError(format!(
                "invalid string when parsing Scaling Technique: '{}'",
                s
            ))),
        }
    }
}
