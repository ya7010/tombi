use ast::Boolean;

use crate::Format;

impl Format for Boolean {
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
    #[case("boolean = true")]
    #[case("boolean = false")]
    fn boolean_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), source);
        assert_eq!(p.errors(), []);
    }
}
