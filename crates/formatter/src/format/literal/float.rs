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

    crate::test_format! {
        #[test]
        fn float_key_value1("key = +1.0");

        #[test]
        fn float_key_value2("key = 3.1415");

        #[test]
        fn float_key_value3("key = -0.01");

        #[test]
        fn float_key_value4("key = 5e+22");

        #[test]
        fn float_key_value5("key = 1e06");

        #[test]
        fn float_key_value6("key = -2E-2");

        #[test]
        fn float_key_value7("key = 6.626e-34");

        #[test]
        fn float_key_value8("key = 224_617.445_991_228");
    }

    crate::test_format! {
        #[test]
        fn invalid_key_value1("invalid_float_1 = .7") -> Err;

        #[test]
        fn invalid_key_value2("invalid_float_2 = 7.") -> Err;

        #[test]
        fn invalid_key_value3("invalid_float_3 = 3.e+20") -> Err;
    }
}
