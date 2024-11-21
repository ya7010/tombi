#[derive(Debug, Default, Clone, Copy, schemars::JsonSchema)]
pub enum LineEnding {
    #[default]
    Lf,
    Crlf,
}

impl serde::Serialize for LineEnding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LineEnding::Lf => serializer.serialize_str("lf"),
            LineEnding::Crlf => serializer.serialize_str("crlf"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for LineEnding {
    fn deserialize<D>(deserializer: D) -> Result<LineEnding, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "lf" => Ok(LineEnding::Lf),
            "crlf" => Ok(LineEnding::Crlf),
            _ => Err(serde::de::Error::custom("invalid line ending")),
        }
    }
}
