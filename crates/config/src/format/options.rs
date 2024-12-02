use crate::format::DateTimeDelimiter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct FormatOptions {
    /// # The type of line ending.
    ///
    /// In TOML, the line ending must be either `LF` or `CRLF`.
    ///
    /// - `lf`: Line Feed only (`\n`), common on Linux and macOS as well as inside git repos.
    /// - `crlf`: Carriage Return Line Feed (`\r\n`), common on Windows.
    pub line_ending: Option<crate::format::LineEnding>,

    /// # The delimiter between date and time.
    ///
    /// In accordance with [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339), you can use `T` or space character between date and time.
    ///
    /// - `T`: Example: `2001-01-01T00:00:00`
    /// - `space`: Example: `2001-01-01 00:00:00`
    /// - `preserve`: Preserve the original delimiter.
    pub date_time_delimiter: Option<DateTimeDelimiter>,
}

impl FormatOptions {
    pub fn merge(&mut self, other: &FormatOptions) -> &mut Self {
        if let Some(line_ending) = other.line_ending {
            self.line_ending = Some(line_ending);
        }
        if let Some(date_time_delimiter) = other.date_time_delimiter {
            self.date_time_delimiter = Some(date_time_delimiter);
        }

        self
    }

    #[inline]
    pub fn date_time_delimiter(&self) -> DateTimeDelimiter {
        self.date_time_delimiter.unwrap_or_default()
    }
}
