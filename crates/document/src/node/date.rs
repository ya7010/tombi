use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Date {
    value: String,
    range: crate::Range,
}

impl Date {
    pub fn new_local_date(source: &str, node: ast::LocalDate) -> Self {
        Self {
            value: node.to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn range(&self) -> crate::Range {
        self.range
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
