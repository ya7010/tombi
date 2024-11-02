use crate::Format;

impl Format for ast::InlineTable {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        let entries = self
            .entries()
            .map(|it| it.format(context))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{ {} }}", entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::AstNode;
    use rstest::rstest;

    #[rstest]
    #[case(r#"name = { first = "Tom", last = "Preston-Werner" }"#)]
    #[case(r#"point = { x = 1, y = 2 }"#)]
    #[case(r#"animal = { type.name = "pug" }"#)]
    fn inline_table_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        assert_eq!(p.errors(), []);
        assert_eq!(ast.format_default(), source);
    }
}
