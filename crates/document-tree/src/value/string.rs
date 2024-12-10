#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKind {
    BasicString,
    LiteralString,
    MultiLineBasicString,
    MultiLineLiteralString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    kind: StringKind,
    value: std::string::String,
    range: text::Range,
}

impl String {
    #[inline]
    pub fn kind(&self) -> StringKind {
        self.kind
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::BasicString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::BasicString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::BasicString,
            value: token.text().to_string(),
            range: token.range(),
        })
    }
}

impl TryFrom<ast::LiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::LiteralString,
            value: token.text().to_string(),
            range: token.range(),
        })
    }
}

impl TryFrom<ast::MultiLineBasicString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineBasicString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::MultiLineBasicString,
            value: token.text().to_string(),
            range: token.range(),
        })
    }
}

impl TryFrom<ast::MultiLineLiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineLiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::MultiLineLiteralString,
            value: token.text().to_string(),
            range: token.range(),
        })
    }
}
