#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone)]
pub struct SchemaOptions {
    /// # The schema URL.
    url: String,

    /// # The file match condition to apply the schema.
    file_match: FileMatch,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum FileMatch {
    Pattern(FilePattern),
    #[cfg_attr(feature = "jsonschema", schemars(range(min = 1)))]
    Patterns(Vec<FilePattern>),
}

/// # File match pattern of the schema.
///
/// Supports glob pattern.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum FilePattern {
    /// A glob pattern.
    Glob(String),
}
