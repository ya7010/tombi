use toml_version::TomlVersion;

use crate::{OneOrMany, SchemaCatalogEnabled, SchemaCatalogPath};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct SchemaOptions {
    /// # Schema catalog options.
    pub catalog: Option<SchemaCatalog>,
}

impl SchemaOptions {
    pub const fn default() -> Self {
        Self { catalog: None }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct SchemaCatalog {
    /// # Enable or disable the schema catalog.
    pub enabled: Option<SchemaCatalogEnabled>,

    /// # The schema catalog path or url.
    #[cfg_attr(
        feature = "jsonschema",
        schemars(default = "SchemaCatalogPath::default")
    )]
    pub path: Option<OneOrMany<SchemaCatalogPath>>,
}

impl SchemaCatalog {
    pub fn paths(&self) -> Option<Vec<SchemaCatalogPath>> {
        if self.enabled.unwrap_or_default().value() {
            match &self.path {
                Some(path) => Some(
                    path.as_ref()
                        .into_iter()
                        .map(Clone::clone)
                        .collect::<Vec<_>>(),
                ),
                None => Some(vec![SchemaCatalogPath::default()]),
            }
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone)]
pub struct SchemaCatalogItem {
    /// # The TOML version that the schema is available.
    pub toml_version: Option<TomlVersion>,

    /// # The schema path.
    pub path: String,

    /// # The file match pattern of the schema.
    ///
    /// Supports glob pattern.
    #[cfg_attr(feature = "jsonschema", schemars(length(min = 1)))]
    pub include: Option<Vec<String>>,
    // /// # The schema options for specific keys.
    // #[cfg_attr(feature = "jsonschema", schemars(default))]
    // subschemas: Option<Vec<SubSchemaOptions>>,
}

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
// #[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
// #[derive(Debug, Clone)]
// pub struct SubSchemaOptions {
//     /// # The schema path.
//     path: String,

//     /// The path of the key to apply the schema.
//     keys: String,
// }
