use crate::Format;
use std::fmt::Write;

impl Format for ast::BasicString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::MultiLineBasicString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::LiteralString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::MultiLineLiteralString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
