mod local_date;
mod local_date_time;
mod local_time;
mod offset;
mod offset_date_time;
mod private;

use std::error;
use std::fmt;

pub use local_date::LocalDate;
pub use local_date_time::LocalDateTime;
pub use local_time::LocalTime;
pub use offset::Offset;
pub use offset_date_time::OffsetDateTime;

/// Error returned from parsing a `Datetime` in the `FromStr` implementation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DatetimeParseError {}

#[doc(hidden)]
#[cfg(feature = "serde")]
pub const FIELD: &str = "$__tombi_private_datetime";
#[doc(hidden)]
#[cfg(feature = "serde")]
pub const NAME: &str = "$__tombi_private_Datetime";

#[doc(hidden)]
#[cfg(feature = "serde")]
struct DatetimeFromString {
    pub value: crate::private::DateTime,
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for DatetimeFromString {
    fn deserialize<D>(deserializer: D) -> Result<DatetimeFromString, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DatetimeFromString;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("string containing a datetime")
            }

            fn visit_str<E>(self, s: &str) -> Result<DatetimeFromString, E>
            where
                E: serde::de::Error,
            {
                match s.parse() {
                    Ok(date) => Ok(DatetimeFromString { value: date }),
                    Err(e) => Err(serde::de::Error::custom(e)),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl fmt::Display for DatetimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to parse datetime".fmt(f)
    }
}

impl error::Error for DatetimeParseError {}
