use crate::Enabled;

/// # Language Server options.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "jsonschema", schemars(extend("x-tombi-table-keys-order" = x_tombi::TableKeysOrder::Schema)))]
#[derive(Debug, Default, Clone)]
pub struct ServerOptions {
    /// # Hover Feature options.
    pub hover: Option<ServerHover>,

    /// # Completion Feature options.
    pub completion: Option<ServerCompletion>,

    /// # Formatting Feature options.
    pub formatting: Option<ServerFormatting>,

    /// # Diagnostics Feature options.
    pub diagnostics: Option<ServerDiagnostics>,
}

impl ServerOptions {
    pub const fn default() -> Self {
        Self {
            hover: None,
            completion: None,
            formatting: None,
            diagnostics: None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerHover {
    /// # Enable hover feature.
    ///
    /// Whether to enable hover.
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerCompletion {
    /// # Enable completion feature.
    ///
    /// Whether to enable completion.
    ///
    /// **WARNING**: 🚧 This feature is experimental 🚧
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerFormatting {
    /// # Enable formatting feature.
    ///
    /// Whether to enable formatting.
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct ServerDiagnostics {
    /// # Enable diagnostics feature.
    ///
    /// Whether to enable diagnostics.
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,
}

impl ServerCompletion {
    pub const fn default() -> Self {
        Self { enabled: None }
    }
}
