use toml_version::TomlVersion;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone)]
pub struct SchemaOptions {
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
