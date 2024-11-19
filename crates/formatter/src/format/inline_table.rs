use crate::{
    format::comment::{LeadingComment, TailingComment},
    Format,
};
use ast::AstNode;
use std::fmt::Write;

use super::comment::{BeginDanglingComment, EndDanglingComment};

impl Format for ast::InlineTable {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.should_be_multiline() {
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

    write!(f, "{}{{{}", f.ident(), f.line_ending())?;

    f.inc_ident();

    table
        .inner_begin_dangling_comments()
        .map(BeginDanglingComment)
        .collect::<Vec<_>>()
        .fmt(f)?;

    for (i, (entry, comma)) in table.entries_with_comma().enumerate() {
        // value format
        {
            if i > 0 {
                write!(f, "{}", f.line_ending())?;
            }
            entry.fmt(f)?;
        }

        // comma format
        {
            let (comma_leading_comments, comma_tailing_comment) = match comma {
                Some(comma) => (
                    comma.leading_comments().collect::<Vec<_>>(),
                    comma.tailing_comment(),
                ),
                None => (vec![], None),
            };

            if !comma_leading_comments.is_empty() {
                write!(f, "{}", f.line_ending())?;
                for comment in comma_leading_comments {
                    LeadingComment(comment).fmt(f)?;
                }
                write!(f, "{},", f.ident())?;
            } else if entry.tailing_comment().is_some() {
                write!(f, "{}{},", f.line_ending(), f.ident())?;
            } else {
                write!(f, ",")?;
            }

            if let Some(comment) = comma_tailing_comment {
                TailingComment(comment).fmt(f)?;
            }
        }
    }

    table
        .inner_end_dangling_comments()
        .map(EndDanglingComment)
        .collect::<Vec<_>>()
        .fmt(f)?;

    f.dec_ident();

    write!(f, "{}{}}}", f.line_ending(), f.ident())?;

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

    f.with_reset_ident(|f| {
        for (i, entry) in table.entries().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            entry.fmt(f)?;
        }
        Ok(())
    })?;

    write!(f, "{}}}", f.defs().inline_table_brace_inner_space())?;

    if let Some(comment) = table.tailing_comment() {
        TailingComment(comment).fmt(f)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_format;

    use super::*;
    use ast::AstNode;

    test_format! {
        #[test]
        fn inline_table_key_value1(r#"name = { first = "Tom", last = "Preston-Werner" }"#) -> Ok(_);
    }

    test_format! {
        #[test]
        fn inline_table_key_value2(r#"point = { x = 1, y = 2 }"#) -> Ok(_);

    }

    test_format! {
        #[test]
        fn inline_table_key_value3(r#"animal = { type.name = "pug" }"#) -> Ok(_);
    }
}
