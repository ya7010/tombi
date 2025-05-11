use crate::BoolDefaultTrue;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "jsonschema", schemars(extend("x-tombi-table-keys-order" = tombi_x_keyword::TableKeysOrder::Schema)))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LspOptions {
    /// # Hover Feature options.
    pub hover: Option<LspHover>,

    /// # Completion Feature options.
    pub completion: Option<LspCompletion>,

    /// # Formatting Feature options.
    pub formatting: Option<LspFormatting>,

    /// # Diagnostics Feature options.
    pub diagnostics: Option<LspDiagnostics>,

    /// # Goto Declaration Feature options.
    pub goto_declaration: Option<LspGotoDefinition>,

    /// # Goto Definition Feature options.
    pub goto_definition: Option<LspGotoDefinition>,

    /// # Goto Type Definition Feature options.
    pub goto_type_definition: Option<LspGotoDefinition>,
}

impl LspOptions {
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
pub struct LspHover {
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
pub struct LspCompletion {
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
pub struct LspFormatting {
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
pub struct LspDiagnostics {
    /// # Enable diagnostics feature.
    ///
    /// Whether to enable diagnostics.
    pub enabled: Option<BoolDefaultTrue>,
}

impl LspCompletion {
    pub const fn default() -> Self {
        Self { enabled: None }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LspGotoDefinition {
    /// # Enable goto definition feature.
    ///
    /// Whether to enable goto definition.
    pub enabled: Option<BoolDefaultTrue>,
}
