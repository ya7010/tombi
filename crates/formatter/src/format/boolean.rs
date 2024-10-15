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

    #[test]
    fn test_boolean() {
        tracing_subscriber::fmt::init();
        let p = parser::parse("boolean = true");
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), "boolean = true");
    }
}
