use ast::Boolean;

use super::Format;

impl Format for Boolean {
    fn format(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use ast::AstNode;

    #[test]
    fn test_boolean() {
        let p = parser::parse("true");

        let ast = Boolean::cast(p.syntax_node());
        assert_matches!(ast, Some(Boolean { .. }));

        let boolean = ast.unwrap();
        assert_eq!(boolean.format(), "true");
    }
}
