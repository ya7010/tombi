use text::raw_string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKind {
    BasicString(ast::BasicString),
    LiteralString(ast::LiteralString),
    MultiLineBasicString(ast::MultiLineBasicString),
    MultiLineLiteralString(ast::MultiLineLiteralString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    kind: StringKind,
    value: std::string::String,
}

impl String {
    #[inline]
    pub fn kind(&self) -> &StringKind {
        &self.kind
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn raw_string(&self) -> std::string::String {
        match self.kind {
            StringKind::BasicString(_) => raw_string::from_basic_string(&self.value),
            StringKind::LiteralString(_) => raw_string::from_literal_string(&self.value),
            StringKind::MultiLineBasicString(_) => {
                raw_string::from_multi_line_basic_string(&self.value)
            }
            StringKind::MultiLineLiteralString(_) => {
                raw_string::from_multi_line_literal_string(&self.value)
            }
        }
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        match self.kind() {
            StringKind::BasicString(node) => node.token(),
            StringKind::LiteralString(node) => node.token(),
            StringKind::MultiLineBasicString(node) => node.token(),
            StringKind::MultiLineLiteralString(node) => node.token(),
        }
        .unwrap()
        .range()
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.range()
    }
}

impl TryFrom<ast::BasicString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::BasicString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::BasicString(node),
            value: token.text().to_string(),
        })
    }
}

impl TryFrom<ast::LiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::LiteralString(node),
            value: token.text().to_string(),
        })
    }
}

impl TryFrom<ast::MultiLineBasicString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineBasicString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::MultiLineBasicString(node),
            value: token.text().to_string(),
        })
    }
}

impl TryFrom<ast::MultiLineLiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineLiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Ok(Self {
            kind: StringKind::MultiLineLiteralString(node),
            value: token.text().to_string(),
        })
    }
}
