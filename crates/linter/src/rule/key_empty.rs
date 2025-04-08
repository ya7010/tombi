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
            l.extend_diagnostics(crate::Warning {
                kind: crate::WarningKind::KeyEmpty,
                range: node.syntax().range(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use diagnostic::SetDiagnostics;

    #[tokio::test]
    async fn test_key_empty() {
        let diagnostics = crate::Linter::new(
            config::TomlVersion::default(),
            &crate::LintOptions::default(),
            None,
            &schema_store::SchemaStore::new(),
        )
        .lint("'' = 1")
        .await
        .unwrap_err();

        let mut expected = vec![];
        crate::Warning {
            kind: crate::WarningKind::KeyEmpty,
            range: text::Range::new((0, 0).into(), (0, 2).into()),
        }
        .set_diagnostics(&mut expected);

        assert_eq!(diagnostics, expected);
    }
}
