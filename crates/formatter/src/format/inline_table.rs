use crate::{
    format::comment::{LeadingComment, TailingComment},
    Format,
};
use ast::AstNode;
use std::fmt::Write;

impl Format for ast::InlineTable {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_multiline() {
            format_multiline_inline_table(self, f)
        } else {
            format_singleline_inline_table(self, f)
        }
    }
}

fn format_multiline_inline_table(
    table: &ast::InlineTable,
    f: &mut crate::Formatter,
) -> Result<(), std::fmt::Error> {
    for comment in table.leading_comments() {
        LeadingComment(comment).fmt(f)?;
    }

    writeln!(f, "{}{{", f.ident(),)?;

    f.inc_ident();

    let key_values = table.entries().collect::<Vec<_>>();
    for (i, key_value) in key_values.iter().enumerate() {
        if i > 0 {
            writeln!(f, ",")?;
        }
        key_value.fmt(f)?;
    }

    f.dec_ident();

    write!(f, "\n{}}}", f.ident())?;

    if let Some(comment) = table.tailing_comment() {
        TailingComment(comment).fmt(f)?;
    }

    Ok(())
}

fn format_singleline_inline_table(
    table: &ast::InlineTable,
    f: &mut crate::Formatter,
) -> Result<(), std::fmt::Error> {
    for comment in table.leading_comments() {
        LeadingComment(comment).fmt(f)?;
    }

    write!(
        f,
        "{}{{{}",
        f.ident(),
        f.defs().inline_table_brace_inner_space()
    )?;

    let key_values = table.entries().collect::<Vec<_>>();
    for (i, key_value) in key_values.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        key_value.fmt(f)?;
    }

    write!(f, "{}}}", f.defs().inline_table_brace_inner_space())?;

    if let Some(comment) = table.tailing_comment() {
        TailingComment(comment).fmt(f)?;
    }

    Ok(())
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
