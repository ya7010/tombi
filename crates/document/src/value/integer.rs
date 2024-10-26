#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: String,
    range: crate::Range,
}
