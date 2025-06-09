mod version_sort;

pub use version_sort::version_sort;

pub const X_TOMBI_TOML_VERSION: &str = "x-tombi-toml-version";
pub const X_TOMBI_ARRAY_VALUES_ORDER: &str = "x-tombi-array-values-order";
pub const X_TOMBI_TABLE_KEYS_ORDER: &str = "x-tombi-table-keys-order";

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum ArrayValuesOrder {
    Ascending,
    Descending,
    // Version Sorting
    //
    // See: https://doc.rust-lang.org/nightly/style-guide/index.html#sorting
    VersionSort,
}

impl std::fmt::Display for ArrayValuesOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "ascending"),
            Self::Descending => write!(f, "descending"),
            Self::VersionSort => write!(f, "version-sort"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum TableKeysOrder {
    Ascending,
    Descending,
    Schema,
    // Version Sorting
    //
    // See: https://doc.rust-lang.org/nightly/style-guide/index.html#sorting
    VersionSort,
}

impl std::fmt::Display for TableKeysOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableKeysOrder::Ascending => write!(f, "ascending"),
            TableKeysOrder::Descending => write!(f, "descending"),
            TableKeysOrder::Schema => write!(f, "schema"),
            TableKeysOrder::VersionSort => write!(f, "version-sort"),
        }
    }
}
