#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct LocalDateTime {
    date: crate::private::Date,
    time: crate::private::Time,
}

impl LocalDateTime {
    #[cfg(feature = "serde")]
    pub(crate) fn type_name() -> &'static str {
        "local date time"
    }

    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            date: crate::private::Date { year, month, day },
            time: crate::private::Time {
                hour,
                minute,
                second,
                nanosecond: 0,
            },
        }
    }

    pub fn from_ymd_hms_milli(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u32,
    ) -> Self {
        assert!(millisecond < 1_000);

        Self {
            date: crate::private::Date { year, month, day },
            time: crate::private::Time {
                hour,
                minute,
                second,
                nanosecond: millisecond * 1_000_000,
            },
        }
    }

    pub fn from_ymd_hms_nano(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Self {
        assert!(nanosecond < 1_000_000_000);
        Self {
            date: crate::private::Date { year, month, day },
            time: crate::private::Time {
                hour,
                minute,
                second,
                nanosecond,
            },
        }
    }

    pub fn year(&self) -> u16 {
        self.date.year
    }

    pub fn month(&self) -> u8 {
        self.date.month
    }

    pub fn day(&self) -> u8 {
        self.date.day
    }

    pub fn hour(&self) -> u8 {
        self.time.hour
    }

    pub fn minute(&self) -> u8 {
        self.time.minute
    }

    pub fn second(&self) -> u8 {
        self.time.second
    }

    pub fn nanosecond(&self) -> u32 {
        self.time.nanosecond
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        crate::private::DateTime {
            date: Some(self.date),
            time: Some(self.time),
            offset: None,
        }
        .serialize(serializer)
    }
}
