#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {
    pub line_length: u8,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            line_length: DEFAULT_LINE_LENGTH,
        }
    }
}

pub const DEFAULT_LINE_LENGTH: u8 = 100;
pub const DEFAULT_LINE_LENGTH_STR: &'static str = "100";
