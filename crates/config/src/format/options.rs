#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct FormatOptions {
    /// The type of line ending.
    ///
    /// - `lf`: Line Feed only (`\n`), common on Linux and macOS as well as inside git repos.
    /// - `crlf`: Carriage Return Line Feed (`\r\n`), common on Windows.
    pub line_ending: Option<crate::format::LineEnding>,
}

impl FormatOptions {
    pub fn merge(&mut self, other: &FormatOptions) -> &mut Self {
        if let Some(line_ending) = other.line_ending {
            self.line_ending = Some(line_ending);
        }

        self
    }
}
