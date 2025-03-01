use ast::AstNode;

use crate::Rule;

pub struct KeyEmptyRule;

impl Rule<ast::Key> for KeyEmptyRule {
    fn check(node: &ast::Key, l: &mut crate::Linter) {
        if match node {
            ast::Key::BareKey(_) => false,
            ast::Key::BasicString(node) => node.syntax().text() == "\"\"",
            ast::Key::LiteralString(node) => node.syntax().text() == "''",
        } {
            l.add_diagnostic(diagnostic::Diagnostic::new_warning(
                crate::ErrorKind::KeyEmpty.to_string(),
                node.syntax().range(),
            ));
        };
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[tokio::test]
    async fn test_key_empty() {
        let err = crate::Linter::try_new(
            config::TomlVersion::default(),
            &crate::LintOptions::default(),
            None,
            &schema_store::SchemaStore::new(false),
        )
        .await
        .unwrap()
        .lint("'' = 1")
        .await
        .unwrap_err();
        assert_eq!(
            err,
            vec![diagnostic::Diagnostic::new_warning(
                crate::ErrorKind::KeyEmpty.to_string(),
                text::Range::new((0, 0).into(), (0, 2).into()),
            )]
        );
    }
}
