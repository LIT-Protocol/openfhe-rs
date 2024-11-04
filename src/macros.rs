macro_rules! try_from_int_impl {
    ($name:ident, $error:ident, $($type:ty),+$(,)*) => {
        $(
            impl TryFrom<$type> for $name {
                type Error = Error;

                fn try_from(value: $type) -> Result<Self, Self::Error> {
                    u8::try_from(value)
                        .map_err(|_| Error::$error(value as usize))
                        .and_then(|value| Self::try_from(value))
                }
            }
        )+
    };
}

macro_rules! into_int_impl {
    ($name:ident, $($type:ty),+$(,)*) => {
        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                value as u8
            }
        }

        $(
            impl From<$name> for $type {
                fn from(value: $name) -> Self {
                    let out = value as u8;
                    out as $type
                }
            }
        )+
    };
}

macro_rules! enum_serde_impl {
    ($name:ident) => {
        impl serde::Serialize for $name {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                if s.is_human_readable() {
                    s.serialize_str(&self.to_string())
                } else {
                    s.serialize_u8(u8::from(*self))
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(d: D) -> Result<$name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                if d.is_human_readable() {
                    let s = String::deserialize(d)?;
                    $name::from_str(&s).map_err(serde::de::Error::custom)
                } else {
                    let u = u8::deserialize(d)?;
                    $name::try_from(u).map_err(serde::de::Error::custom)
                }
            }
        }
    };
}
