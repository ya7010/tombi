#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float {
    value: String,
    range: crate::Range,
}

impl Float {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn range(&self) -> &crate::Range {
        &self.range
    }
}
