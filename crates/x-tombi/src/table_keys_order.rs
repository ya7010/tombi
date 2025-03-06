#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum TableKeysOrder {
    Ascending,
    Descending,
    Schema,
}

impl std::fmt::Display for TableKeysOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableKeysOrder::Ascending => write!(f, "ascending"),
            TableKeysOrder::Descending => write!(f, "descending"),
            TableKeysOrder::Schema => write!(f, "schema"),
        }
    }
}
