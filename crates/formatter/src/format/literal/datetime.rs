use super::LiteralNode;

impl LiteralNode for ast::OffsetDateTime {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::LocalDateTime {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::LocalDate {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::LocalTime {
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
    #[case("odt1 = 1979-05-27T07:32:00Z")]
    #[case("odt2 = 1979-05-27T00:32:00-07:00")]
    #[case("odt3 = 1979-05-27T00:32:00.999999-07:00")]
    fn valid_offset_datetime_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("ldt1 = 1979-05-27T07:32:00")]
    #[case("ldt2 = 1979-05-27T00:32:00.999999")]
    fn valid_local_datetime_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("ld1 = 1979-05-27")]
    fn valid_local_date_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("lt1 = 07:32:00")]
    #[case("lt2 = 00:32:00.999999")]
    fn valid_local_time_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }
}
