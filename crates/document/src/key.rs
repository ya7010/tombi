use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    value: String,
    range: crate::Range,
}

impl Key {
    pub(crate) fn new(source: &str, key: ast::Key) -> Self {
        Self {
            value: key.to_string(),
            range: Range::from_source(source, key),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn range(&self) -> Range {
        self.range
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
