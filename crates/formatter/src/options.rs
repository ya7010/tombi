#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {
    /// The type of line ending.
    ///
    /// - `lf`: Line Feed only (`\n`), common on Linux and macOS as well as inside git repos.
    /// - `crlf`: Carriage Return Line Feed (`\r\n`), common on Windows.
    pub line_ending: Option<LineEnding>,
}

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

impl Options {
    pub fn merge(&mut self, other: &Options) -> &mut Self {
        if let Some(line_ending) = other.line_ending {
            self.line_ending = Some(line_ending);
        }

        self
    }
}
