#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct LocalDate(crate::private::Date);

impl LocalDate {
    #[cfg(feature = "serde")]
    pub(crate) fn type_name() -> &'static str {
        "local date"
    }

    pub fn from_ymd(year: u16, month: u8, day: u8) -> Self {
        Self(crate::private::Date { year, month, day })
    }

    pub fn year(&self) -> u16 {
        self.0.year
    }

    pub fn month(&self) -> u8 {
        self.0.month
    }

    pub fn day(&self) -> u8 {
        self.0.day
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for LocalDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        crate::private::DateTime::from(self.0).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for LocalDate {
    fn deserialize<D>(deserializer: D) -> Result<LocalDate, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        match crate::private::DateTime::deserialize(deserializer)? {
            crate::private::DateTime {
                date: Some(date),
                time: None,
                offset: None,
            } => Ok(LocalDate(date)),
            datetime => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other(datetime.type_name()),
                &Self::type_name(),
            )),
        }
    }
}
