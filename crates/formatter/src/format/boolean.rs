use ast::Boolean;

use super::Format;

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
    #[case("boolean  = true")]
    #[case("boolean =  true")]
    #[case("boolean   =  true")]
    fn test_boolean(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), "boolean = true");
    }
}
