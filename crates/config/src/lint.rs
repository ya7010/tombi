use crate::UseSchemaCatalog;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct LintOptions {
    /// # Use schema catalog.
    #[cfg_attr(
        feature = "jsonschema",
        schemars(default = "UseSchemaCatalog::default")
    )]
    pub use_schema_catalog: Option<UseSchemaCatalog>,
}

impl LintOptions {
    pub const fn default() -> Self {
        Self {
            use_schema_catalog: None,
        }
    }
}
