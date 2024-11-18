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

    crate::test_format! {
        #[test]
        fn offset_datetime_key_value1("odt1 = 1979-05-27T07:32:00Z");

        #[test]
        fn offset_datetime_key_value2("odt2 = 1979-05-27T00:32:00-07:00");

        #[test]
        fn offset_datetime_key_value3("odt3 = 1979-05-27T00:32:00.999999-07:00");

        #[test]
        fn local_datetime_key_value1("ldt1 = 1979-05-27T07:32:00");

        #[test]
        fn local_datetime_key_value2("ldt2 = 1979-05-27T00:32:00.999999");

        #[test]
        fn valid_local_date_key_value("ld1 = 1979-05-27");

        #[test]
        fn valid_local_time_key_value1("lt1 = 07:32:00");

        #[test]
        fn valid_local_time_key_value2("lt2 = 00:32:00.999999");
    }
}
