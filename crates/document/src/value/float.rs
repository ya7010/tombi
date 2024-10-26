use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float {
    value: String,
    range: crate::Range,
}

impl Float {
    pub fn new(source: &str, value: ast::Float) -> Self {
        Self {
            value: value.to_string(),
            range: Range::from_source(source, value),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn range(&self) -> crate::Range {
        self.range
    }
}
