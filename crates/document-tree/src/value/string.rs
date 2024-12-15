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
    pub fn try_new(
        kind: StringKind,
        value: std::string::String,
    ) -> Result<Self, crate::support::string::ParseError> {
        let string = Self { kind, value };
        string.try_to_raw_string()?;

        Ok(string)
    }

    #[inline]
    pub fn kind(&self) -> &StringKind {
        &self.kind
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn to_raw_string(&self) -> std::string::String {
        // NOTE: String has already been validated by `impl TryIntoDocumentTree<String>`,
        //       so it's safe to unwrap.
        self.try_to_raw_string().unwrap()
    }

    #[inline]
    fn try_to_raw_string(&self) -> Result<std::string::String, crate::support::string::ParseError> {
        match self.kind {
            StringKind::BasicString(_) => {
                crate::support::string::try_from_basic_string(&self.value)
            }
            StringKind::LiteralString(_) => {
                crate::support::string::try_from_literal_string(&self.value)
            }
            StringKind::MultiLineBasicString(_) => {
                crate::support::string::try_from_multi_line_basic_string(&self.value)
            }
            StringKind::MultiLineLiteralString(_) => {
                crate::support::string::try_from_multi_line_literal_string(&self.value)
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

        Self::try_new(StringKind::BasicString(node), token.text().to_string()).map_err(|error| {
            vec![crate::Error::ParseStringError {
                error,
                range: token.range(),
            }]
        })
    }
}

impl TryFrom<ast::LiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Self::try_new(StringKind::LiteralString(node), token.text().to_string()).map_err(|error| {
            vec![crate::Error::ParseStringError {
                error,
                range: token.range(),
            }]
        })
    }
}

impl TryFrom<ast::MultiLineBasicString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineBasicString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Self::try_new(
            StringKind::MultiLineBasicString(node),
            token.text().to_string(),
        )
        .map_err(|error| {
            vec![crate::Error::ParseStringError {
                error,
                range: token.range(),
            }]
        })
    }
}

impl TryFrom<ast::MultiLineLiteralString> for String {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::MultiLineLiteralString) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();

        Self::try_new(
            StringKind::MultiLineLiteralString(node),
            token.text().to_string(),
        )
        .map_err(|error| {
            vec![crate::Error::ParseStringError {
                error,
                range: token.range(),
            }]
        })
    }
}
