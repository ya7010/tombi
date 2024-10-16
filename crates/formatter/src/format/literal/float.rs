use crate::Format;

impl Format for ast::Float {
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
    #[case("key = +1.0", "+1.0")]
    #[case("key  = 3.1415", "3.1415")]
    #[case("key = -0.01", "-0.01")]
    #[case("key = 5e+22", "5e+22")]
    #[case("key = 1e06", "1e06")]
    #[case("key = -2E-2", "-2E-2")]
    #[case("key = 6.626e-34", "6.626e-34")]
    #[case("key = 224_617.445_991_228", "224_617.445_991_228")]
    fn valid_float_key_value(#[case] source: &str, #[case] value: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), format!("key = {value}"));
        assert_eq!(p.errors().len(), 0);
    }

    #[rstest]
    #[case("invalid_float_1 = .7")]
    #[case("invalid_float_2 = 7.")]
    #[case("invalid_float_3 = 3.e+20")]
    fn invalid_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        assert_ne!(p.errors().len(), 0);
    }
}
