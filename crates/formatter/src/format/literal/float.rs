use super::LiteralNode;

impl LiteralNode for ast::Float {
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
    #[case("key = +1.0")]
    #[case("key = 3.1415")]
    #[case("key = -0.01")]
    #[case("key = 5e+22")]
    #[case("key = 1e06")]
    #[case("key = -2E-2")]
    #[case("key = 6.626e-34")]
    #[case("key = 224_617.445_991_228")]
    fn valid_float_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("invalid_float_1 = .7")]
    #[case("invalid_float_2 = 7.")]
    #[case("invalid_float_3 = 3.e+20")]
    fn invalid_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        assert_ne!(p.errors(), vec![]);
    }
}
