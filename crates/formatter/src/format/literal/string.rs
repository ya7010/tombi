use super::LiteralNode;

impl LiteralNode for ast::BasicString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::MultiLineBasicString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::LiteralString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::MultiLineLiteralString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

#[cfg(test)]
mod tests {
    use crate::Format;
    use ast::AstNode;

    crate::test_format! {
        #[test]
        fn basic_string_key_value1(r#"key = "value""#) -> r#"key = "value""#;

        #[test]
        fn basic_string_key_value2(r#"key    = "value""#) -> r#"key = "value""#;
    }
}
