use crate::Format;
use std::fmt::Write;

impl Format for ast::IntegerBin {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::IntegerHex {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::IntegerDec {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Format for ast::IntegerOct {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use ast::AstNode;
    use rstest::rstest;

    use crate::format::Format;

    #[rstest]
    #[case("int1 = +99")]
    #[case("int2 = 42")]
    #[case("int3 = 0")]
    #[case("int4 = -17")]
    #[case("int5 = 1_000")]
    #[case("int6 = 5_349_221")]
    #[case("int7 = 53_49_221")]
    #[case("int8 = 1_2_3_4_5")]
    fn valid_integer_dec_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("hex1 = 0xDEADBEEF")]
    #[case("hex2 = 0xdeadbeef")]
    #[case("hex3 = 0xdead_beef")]
    fn valid_integer_hex_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("oct1 = 0o01234567")]
    #[case("oct2 = 0o755")]
    fn valid_integer_oct_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("bin1 = 0b11010110")]
    #[case("bin2 = 0b1101_0110")]
    fn valid_integer_bin_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }
}
