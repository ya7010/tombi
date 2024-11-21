#[derive(Debug, Default, Clone, Copy, schemars::JsonSchema)]
pub enum DateTimeDelimiter {
    #[default]
    T,
    Space,
}

impl serde::Serialize for DateTimeDelimiter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DateTimeDelimiter::T => serializer.serialize_str("T"),
            DateTimeDelimiter::Space => serializer.serialize_str(" "),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateTimeDelimiter {
    fn deserialize<D>(deserializer: D) -> Result<DateTimeDelimiter, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "T" => Ok(DateTimeDelimiter::T),
            " " => Ok(DateTimeDelimiter::Space),
            _ => Err(serde::de::Error::custom("invalid date time delimiter")),
        }
    }
}
