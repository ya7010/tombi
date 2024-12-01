use crate::Rule;
use ast::AstNode;

pub struct KeyEmptyRule;

impl Rule<ast::Key> for KeyEmptyRule {
    fn check(node: &ast::Key, l: &mut crate::Linter) {
        if match node {
            ast::Key::BareKey(_) => false,
            ast::Key::BasicString(node) => node.syntax().text() == "\"\"",
            ast::Key::LiteralString(node) => node.syntax().text() == "''",
        } {
            l.add_diagnostic(diagnostic::Diagnostic::new_warning(
                "An empty quoted key is allowed, but it is not recommended",
                node.syntax().text_range(),
            ));
        };
    }
}
