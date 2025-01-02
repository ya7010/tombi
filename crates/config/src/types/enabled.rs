#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Enabled(pub bool);

impl Enabled {
    #[inline]
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Default for Enabled {
    fn default() -> Self {
        Self(true)
    }
}

impl From<bool> for Enabled {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
