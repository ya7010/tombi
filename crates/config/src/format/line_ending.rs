#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum LineEnding {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "lf"))]
    Lf,

    #[cfg_attr(feature = "serde", serde(rename = "crlf"))]
    Crlf,
}
