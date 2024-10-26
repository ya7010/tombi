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
