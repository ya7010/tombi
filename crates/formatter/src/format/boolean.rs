use ast::Boolean;

use super::Format;

impl Format for Boolean {
    fn write_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::AstNode;
    use logos::Logos;
    use rstest::rstest;
    use syntax::{SyntaxKind, SyntaxNode};

    #[test]
    fn test_boolean() {
        let mut lex = SyntaxKind::lexer("true");
        Boolean::cast(SyntaxNode::new_root(lex));
        let actual = format!("{}", input);
        assert_eq!(actual, expected);
    }
}
