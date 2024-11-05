use crate::Format;
use std::fmt::Write;

impl Format for ast::InlineTable {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}{{{}",
            f.ident(),
            f.defs().inline_table_brace_inner_space()
        )?;

        let key_values = self.entries().collect::<Vec<_>>();
        for (i, key_value) in key_values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            key_value.fmt(f)?;
        }

        write!(f, "{}}}", f.defs().inline_table_brace_inner_space())
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

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }
}
