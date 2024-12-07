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

impl From<ast::BasicString> for String {
    fn from(node: ast::BasicString) -> Self {
        let token = node.token().unwrap();
        let text = token.text();

        Self {
            kind: StringKind::BasicString,
            value: text[1..text.len() - 1].replace(r#"\""#, "\""),
            range: token.text_range(),
        }
    }
}

impl From<ast::LiteralString> for String {
    fn from(node: ast::LiteralString) -> Self {
        let token = node.token().unwrap();
        let text = token.text();

        Self {
            kind: StringKind::LiteralString,
            value: text[1..text.len() - 1].replace(r#"\'"#, "'"),
            range: token.text_range(),
        }
    }
}

impl From<ast::MultiLineBasicString> for String {
    fn from(node: ast::MultiLineBasicString) -> Self {
        let token = node.token().unwrap();
        let text = token.text();

        Self {
            kind: StringKind::MultiLineBasicString,
            value: text[3..text.len() - 3].to_string(),
            range: token.text_range(),
        }
    }
}

impl From<ast::MultiLineLiteralString> for String {
    fn from(node: ast::MultiLineLiteralString) -> Self {
        let token = node.token().unwrap();
        let text = token.text();

        Self {
            kind: StringKind::MultiLineLiteralString,
            value: text[3..text.len() - 3].to_string(),
            range: token.text_range(),
        }
    }
}
