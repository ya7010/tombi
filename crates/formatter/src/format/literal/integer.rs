use super::LiteralNode;

impl LiteralNode for ast::IntegerBin {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::IntegerHex {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::IntegerDec {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::IntegerOct {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_format, Format};
    use ast::AstNode;

    test_format! {
        #[test]
        fn integer_bin_key_value1("bin1 = 0b11010110") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_bin_key_value2("bin2 = 0b1101_0110") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_oct_key_value1("oct1 = 0o01234567") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_oct_key_value2("oct2 = 0o755") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_hex_key_value1("hex1 = 0xDEADBEEF") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_hex_key_value2("hex2 = 0xdeadbeef") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_hex_key_value3("hex3 = 0xdead_beef") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value1("int1 = +99") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value2("int2 = 42") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value3("int3 = 0") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value4("int4 = -17") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value5("int5 = 1_000") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value6("int6 = 5_349_221") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value7("int7 = 53_49_221") -> Ok(_);
    }

    test_format! {
        #[test]
        fn integer_dec_key_value8("int8 = 1_2_3_4_5") -> Ok(_);
    }
}
