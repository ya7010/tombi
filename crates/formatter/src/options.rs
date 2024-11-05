#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {
    /// Maximum length of a line.
    pub max_line_length: Option<u8>,
}

impl Options {
    pub fn merge(&mut self, other: &Options) -> &mut Self {
        if other.max_line_length.is_some() {
            self.max_line_length = other.max_line_length;
        }

        self
    }
}
