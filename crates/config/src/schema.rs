use toml_version::TomlVersion;

use crate::{Enabled, OneOrMany, SchemaCatalogPath};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct SchemaOptions {
    /// # Enable or disable the schema.
    #[cfg_attr(feature = "jsonschema", schemars(default = "Enabled::default"))]
    pub enabled: Option<Enabled>,

    /// # Schema catalog options.
    pub catalog: Option<SchemaCatalog>,
}

impl SchemaOptions {
    pub const fn default() -> Self {
        Self {
            enabled: None,
            catalog: None,
        }
    }

    pub fn catalog_paths(&self) -> Option<Vec<SchemaCatalogPath>> {
        if self.enabled.unwrap_or_default().value() {
            Some(
                self.catalog
                    .as_ref()
                    .and_then(|catalog| catalog.path.as_ref().map(|path| path.as_ref().to_vec()))
                    .unwrap_or_else(|| vec![SchemaCatalogPath::default()]),
            )
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct SchemaCatalog {
    /// # The schema catalog path or url.
    ///
    /// You can specify multiple catalogs by making it an array.
    /// If you want to disable the default catalog, specify an empty array.
    #[cfg_attr(
        feature = "jsonschema",
        schemars(default = "SchemaCatalogPath::default")
    )]
    pub path: Option<OneOrMany<SchemaCatalogPath>>,
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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn schema_catalog_paths_default() {
        let schema = SchemaOptions::default();
        let _expected = [SchemaCatalogPath::default()];

        assert_matches!(schema.catalog_paths(), Some(_expected));
    }

    #[test]
    fn schema_catalog_paths_empty() {
        let schema = SchemaOptions {
            catalog: Some(SchemaCatalog {
                path: Some(vec![].into()),
            }),
            ..Default::default()
        };

        let _expected: Vec<SchemaCatalogPath> = vec![];

        assert_matches!(schema.catalog_paths(), Some(_expected));
    }
}
