#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKind {
    BasicString,
    LiteralString,
    MultiLineBasicString,
    MultiLineLiteralString,
}

impl From<document_tree::StringKind> for StringKind {
    fn from(kind: document_tree::StringKind) -> Self {
        match kind {
            document_tree::StringKind::BasicString => Self::BasicString,
            document_tree::StringKind::LiteralString => Self::LiteralString,
            document_tree::StringKind::MultiLineBasicString => Self::MultiLineBasicString,
            document_tree::StringKind::MultiLineLiteralString => Self::MultiLineLiteralString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    kind: StringKind,
    value: std::string::String,
}

impl String {
    #[inline]
    pub fn new(kind: StringKind, value: std::string::String) -> Self {
        Self { kind, value }
    }

    #[inline]
    pub fn kind(&self) -> StringKind {
        self.kind
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<document_tree::String> for String {
    fn from(node: document_tree::String) -> Self {
        Self {
            kind: node.kind().into(),
            value: node.value().to_string(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.kind {
            StringKind::BasicString => self.value[1..self.value.len() - 1].replace(r#"\""#, r#"""#),
            StringKind::LiteralString => self.value[1..self.value.len() - 1].replace(r#"\'"#, "'"),
            StringKind::MultiLineBasicString => self.value[3..self.value.len() - 3].to_string(),
            StringKind::MultiLineLiteralString => self.value[3..self.value.len() - 3].to_string(),
        }
        .serialize(serializer)
    }
}
