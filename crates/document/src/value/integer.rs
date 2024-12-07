#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

impl From<document_tree::IntegerKind> for IntegerKind {
    fn from(kind: document_tree::IntegerKind) -> Self {
        match kind {
            document_tree::IntegerKind::Binary => Self::Binary,
            document_tree::IntegerKind::Decimal => Self::Decimal,
            document_tree::IntegerKind::Octal => Self::Octal,
            document_tree::IntegerKind::Hexadecimal => Self::Hexadecimal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: isize,
}

impl Integer {
    #[inline]
    pub fn kind(&self) -> IntegerKind {
        self.kind
    }

    #[inline]
    pub fn value(&self) -> isize {
        self.value
    }
}

impl From<document_tree::Integer> for Integer {
    fn from(node: document_tree::Integer) -> Self {
        Self {
            kind: node.kind().into(),
            value: node.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
