use crate::Format;

impl Format for ast::OffsetDateTime {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

impl Format for ast::LocalDateTime {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

impl Format for ast::LocalDate {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

impl Format for ast::LocalTime {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::AstNode;
    use rstest::rstest;

    #[rstest]
    #[case("odt1 = 1979-05-27T07:32:00Z")]
    #[case("odt2 = 1979-05-27T00:32:00-07:00")]
    #[case("odt3 = 1979-05-27T00:32:00.999999-07:00")]
    fn valid_offset_datetime_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), format!("{source}"));
        assert_eq!(p.errors().len(), 0);
    }

    #[rstest]
    #[case("ldt1 = 1979-05-27T07:32:00")]
    #[case("ldt2 = 1979-05-27T00:32:00.999999")]
    fn valid_local_datetime_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), format!("{source}"));
        assert_eq!(p.errors().len(), 0);
    }

    #[rstest]
    #[case("ld1 = 1979-05-27")]
    fn valid_local_date_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), format!("{source}"));
        assert_eq!(p.errors().len(), 0);
    }

    #[rstest]
    #[case("lt1 = 07:32:00")]
    #[case("lt2 = 00:32:00.999999")]
    fn valid_local_time_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), format!("{source}"));
        assert_eq!(p.errors().len(), 0);
    }
}
