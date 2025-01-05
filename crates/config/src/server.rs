#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerOptions {
    /// # Enable completion.
    ///
    /// Whether to enable completion.
    ///
    /// **WARNING**: 🚧 This feature is experimental 🚧
    pub completion: Option<bool>,
}

impl ServerOptions {
    pub const fn default() -> Self {
        Self { completion: None }
    }
}
