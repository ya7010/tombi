#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct SchemaCatalogEnabled(pub bool);

impl SchemaCatalogEnabled {
    #[inline]
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Default for SchemaCatalogEnabled {
    fn default() -> Self {
        Self(true)
    }
}

impl From<bool> for SchemaCatalogEnabled {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
