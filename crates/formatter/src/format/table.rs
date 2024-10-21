use crate::Format;

impl Format for ast::Table {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        let header = self.header().unwrap().format(context);
        let key_values = self.key_values().collect::<Vec<_>>();

        if key_values.is_empty() {
            format!("[{header}]")
        } else {
            let key_values = key_values
                .iter()
                .map(|it| it.format(context))
                .collect::<Vec<_>>()
                .join("\n");
            format!("[{header}]\n{key_values}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::AstNode;
    use rstest::rstest;

    #[rstest]
    #[case(r#"[package]"#)]
    #[case(r#"[dependencies."unicase"]"#)]
    #[case(r#"[dependencies.unicase]"#)]
    fn table_only_header(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        assert_eq!(p.errors(), []);
        assert_eq!(ast.format_default(), source);
    }

    #[rstest]
    #[case(
        r#"
[package]
name = "toml-rs"
cli.version = "0.4.0"
"#.trim()
    )]
    fn table(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        assert_eq!(p.errors(), []);
        assert_eq!(ast.format_default(), source);
    }
}
