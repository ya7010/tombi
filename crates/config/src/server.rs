use crate::Enabled;

/// # Language Server options.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerOptions {
    /// # Completion Feature options.
    pub completion: Option<ServerCompletion>,
}

impl ServerOptions {
    pub const fn default() -> Self {
        Self { completion: None }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerCompletion {
    /// # Enable completion.
    ///
    /// Whether to enable completion.
    ///
    /// **WARNING**: 🚧 This feature is experimental 🚧
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,
}

impl ServerCompletion {
    pub const fn default() -> Self {
        Self { enabled: None }
    }
}
