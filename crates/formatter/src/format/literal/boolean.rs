use ast::Boolean;

use super::LiteralNode;

impl LiteralNode for Boolean {
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
        fn boolean_true(r#"boolean = true"#) -> Ok(_);

        #[test]
        fn boolean_false(r#"boolean = false"#) -> Ok(_);
    }
}
