#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {
    pub max_line_length: Option<u8>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_line_length: Default::default(),
        }
    }
}
