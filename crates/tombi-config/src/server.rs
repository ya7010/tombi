use crate::BoolDefaultTrue;

/// # Language Server options.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "jsonschema", schemars(extend("x-tombi-table-keys-order" = tombi_x_keyword::TableKeysOrder::Schema)))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerOptions {
    /// # Hover Feature options.
    pub hover: Option<ServerHover>,

    /// # Completion Feature options.
    pub completion: Option<ServerCompletion>,

    /// # Formatting Feature options.
    pub formatting: Option<ServerFormatting>,

    /// # Diagnostics Feature options.
    pub diagnostics: Option<ServerDiagnostics>,

    /// # Goto Declaration Feature options.
    pub goto_declaration: Option<ServerGotoDefinition>,

    /// # Goto Definition Feature options.
    pub goto_definition: Option<ServerGotoDefinition>,

    /// # Goto Type Definition Feature options.
    pub goto_type_definition: Option<ServerGotoDefinition>,
}

impl ServerOptions {
    pub const fn default() -> Self {
        Self {
            hover: None,
            completion: None,
            formatting: None,
            diagnostics: None,
            goto_declaration: None,
            goto_definition: None,
            goto_type_definition: None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerHover {
    /// # Enable hover feature.
    ///
    /// Whether to enable hover.
    pub enabled: Option<BoolDefaultTrue>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerCompletion {
    /// # Enable completion feature.
    ///
    /// Whether to enable completion.
    pub enabled: Option<BoolDefaultTrue>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerFormatting {
    /// # Enable formatting feature.
    ///
    /// Whether to enable formatting.
    pub enabled: Option<BoolDefaultTrue>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerDiagnostics {
    /// # Enable diagnostics feature.
    ///
    /// Whether to enable diagnostics.
    pub enabled: Option<BoolDefaultTrue>,
}

impl ServerCompletion {
    pub const fn default() -> Self {
        Self { enabled: None }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerGotoDefinition {
    /// # Enable goto definition feature.
    ///
    /// Whether to enable goto definition.
    pub enabled: Option<BoolDefaultTrue>,
}
