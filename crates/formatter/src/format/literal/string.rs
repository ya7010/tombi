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
    use rstest::rstest;

    #[rstest]
    #[case(r#"key = "value\""#)]
    #[case(r#"key    = "value\""#)]
    fn barestring_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, r#"key = "value\""#);
        assert_eq!(p.errors(), []);
    }
}
