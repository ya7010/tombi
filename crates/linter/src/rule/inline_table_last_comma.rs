use crate::Rule;
use ast::AstNode;
use config::TomlVersion;

pub struct InlineTableLastCommaRule;

impl Rule<ast::InlineTable> for InlineTableLastCommaRule {
    fn check(node: &ast::InlineTable, l: &mut crate::Linter) {
        if l.toml_version() >= TomlVersion::V1_1_0_Preview {
            return;
        }

        if let Some((_, Some(_))) = node.key_values_with_comma().last() {
            l.add_diagnostic(diagnostic::Diagnostic::new_error(
                "Trailing comma in inline table",
                node.syntax().text_range(),
            ));
        }
    }
}
