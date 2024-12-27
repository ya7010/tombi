#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct UseSchemaCatalog(pub bool);

impl UseSchemaCatalog {
    #[inline]
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Default for UseSchemaCatalog {
    fn default() -> Self {
        Self(true)
    }
}

impl From<bool> for UseSchemaCatalog {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
