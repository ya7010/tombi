use crate::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKind {
    Basic,
    Literal,
    MultiLineBasic,
    MultiLineLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    kind: StringKind,
    value: std::string::String,
    range: crate::Range,
}

impl String {
    pub fn new_basic_string(source: &str, string: ast::BasicString) -> Self {
        Self {
            kind: StringKind::Basic,
            value: string.to_string(),
            range: Range::from_source(source, string),
        }
    }

    pub fn new_literal_string(source: &str, string: ast::LiteralString) -> Self {
        Self {
            kind: StringKind::Literal,
            value: string.to_string(),
            range: Range::from_source(source, string),
        }
    }

    pub fn new_multi_line_basic_string(source: &str, string: ast::MultiLineBasicString) -> Self {
        Self {
            kind: StringKind::MultiLineBasic,
            value: string.to_string(),
            range: Range::from_source(source, string),
        }
    }

    pub fn new_multi_line_literal_string(
        source: &str,
        string: ast::MultiLineLiteralString,
    ) -> Self {
        Self {
            kind: StringKind::MultiLineLiteral,
            value: string.to_string(),
            range: Range::from_source(source, string),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn range(&self) -> Range {
        self.range
    }
}
