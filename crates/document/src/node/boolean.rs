use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    value: String,
    range: crate::Range,
}

impl Boolean {
    pub(crate) fn new(source: &str, value: ast::Boolean) -> Self {
        Self {
            value: value.token().unwrap().to_string(),
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
