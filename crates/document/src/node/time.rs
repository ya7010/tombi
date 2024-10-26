use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Time {
    value: String,
    range: crate::Range,
}

impl Time {
    pub fn new_local_time(source: &str, node: ast::LocalTime) -> Self {
        Self {
            value: node.to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn range(&self) -> crate::Range {
        self.range
    }
}
