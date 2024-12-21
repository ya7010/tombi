use document_tree::support;

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
            document_tree::StringKind::BasicString(_) => Self::BasicString,
            document_tree::StringKind::LiteralString(_) => Self::LiteralString,
            document_tree::StringKind::MultiLineBasicString(_) => Self::MultiLineBasicString,
            document_tree::StringKind::MultiLineLiteralString(_) => Self::MultiLineLiteralString,
        }
    }
}

impl From<&document_tree::StringKind> for StringKind {
    fn from(kind: &document_tree::StringKind) -> Self {
        match kind {
            document_tree::StringKind::BasicString(_) => Self::BasicString,
            document_tree::StringKind::LiteralString(_) => Self::LiteralString,
            document_tree::StringKind::MultiLineBasicString(_) => Self::MultiLineBasicString,
            document_tree::StringKind::MultiLineLiteralString(_) => Self::MultiLineLiteralString,
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
            StringKind::BasicString => support::string::try_from_basic_string(&self.value),
            StringKind::LiteralString => support::string::try_from_literal_string(&self.value),
            StringKind::MultiLineBasicString => {
                support::string::try_from_multi_line_basic_string(&self.value)
            }
            StringKind::MultiLineLiteralString => {
                support::string::try_from_multi_line_literal_string(&self.value)
            }
        }
        .map_err(|err| serde::ser::Error::custom(err))?
        .serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::test_serialize;

    test_serialize!(
        #[test]
        fn string_us(
            r#"
            string-us   = "null"
            "#
        ) -> Err([
            ("invalid string: invalid control character in input", ((0, 14), (0, 21)))
        ])
    );
}
