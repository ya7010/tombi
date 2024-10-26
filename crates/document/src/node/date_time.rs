use crate::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeKind {
    Offset,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime {
    kind: DateTimeKind,
    value: String,
    range: crate::Range,
}

impl DateTime {
    pub fn new_offset_date_time(source: &str, node: ast::OffsetDateTime) -> Self {
        Self {
            kind: DateTimeKind::Offset,
            value: node.to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn new_local_date_time(source: &str, node: ast::LocalDateTime) -> Self {
        Self {
            kind: DateTimeKind::Local,
            value: node.to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn range(&self) -> crate::Range {
        self.range
    }
}
