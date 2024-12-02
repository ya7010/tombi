/// DateTime delimiter
#[derive(
    Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum DateTimeDelimiter {
    /// Example: `2021-01-01T00:00:00`
    #[default]
    #[serde(rename = "T")]
    T,

    /// Example: `2021-01-01 00:00:00`
    #[serde(rename = "space")]
    Space,

    /// Preserve the original delimiter
    #[serde(rename = "preserve")]
    Preserve,
}
