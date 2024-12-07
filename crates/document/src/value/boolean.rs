#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }
}

impl From<document_tree::Boolean> for Boolean {
    fn from(node: document_tree::Boolean) -> Self {
        Self {
            value: node.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
