use std::fmt::Write;

use ast::{
    BeginDanglingComment, DanglingComment, EndDanglingComment, LeadingComment, TailingComment,
};

use super::Format;

impl Format for Vec<Vec<DanglingComment>> {
    #[inline]
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for (i, comments) in self.iter().enumerate() {
            assert!(!comments.is_empty());
            if i != 0 {
                write!(f, "{}{}", f.line_ending(), f.line_ending())?;
            }

            for (j, comment) in comments.iter().enumerate() {
                if j == 0 {
                    f.write_indent()?;
                    format_comment(f, comment.as_ref(), true)?;
                } else {
                    write!(f, "{}", f.line_ending())?;
                    f.write_indent()?;
                    format_comment(f, comment.as_ref(), false)?;
                }
            }
        }
        Ok(())
    }
}

impl Format for Vec<Vec<BeginDanglingComment>> {
    #[inline]
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comments in self {
            assert!(!comments.is_empty());

            for (i, comment) in comments.iter().enumerate() {
                f.write_indent()?;
                if i == 0 {
                    format_comment(f, comment.as_ref(), true)?;
                } else {
                    format_comment(f, comment.as_ref(), false)?;
                }
                write!(f, "{}", f.line_ending())?;
            }
            write!(f, "{}", f.line_ending())?;
        }

        Ok(())
    }
}

impl Format for Vec<Vec<EndDanglingComment>> {
    #[inline]
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for (i, comments) in self.iter().enumerate() {
            if i != 0 {
                write!(f, "{}", f.line_ending())?;
            }

            for (j, comment) in comments.iter().enumerate() {
                write!(f, "{}", f.line_ending())?;
                f.write_indent()?;
                if j == 0 {
                    format_comment(f, comment.as_ref(), true)?;
                } else {
                    format_comment(f, comment.as_ref(), false)?;
                }
            }
        }
        Ok(())
    }
}

impl Format for Vec<LeadingComment> {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for (i, comment) in self.iter().enumerate() {
            f.write_indent()?;
            if i == 0 {
                format_comment(f, comment.as_ref(), true)?;
            } else {
                format_comment(f, comment.as_ref(), false)?;
            }
            write!(f, "{}", f.line_ending())?;
        }
        Ok(())
    }
}

impl Format for TailingComment {
    #[inline]
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", f.tailing_comment_space())?;
        format_comment(f, self.as_ref(), true)
    }
}

fn format_comment(
    f: &mut crate::Formatter,
    comment: &ast::Comment,
    strip_leading_spaces: bool,
) -> Result<(), std::fmt::Error> {
    let comment = comment.to_string();
    let mut iter = comment.trim_ascii_end().chars();

    // write '#' character
    write!(f, "{}", iter.next().unwrap())?;

    if let Some(c) = iter.next() {
        if c != ' ' && c != '\t' {
            write!(f, " ")?;
        }
        write!(f, "{}", c)?;
    }
    if strip_leading_spaces {
        for c in iter.by_ref() {
            if c != ' ' && c != '\t' {
                write!(f, "{}", c)?;
                break;
            }
        }
    }

    write!(f, "{}", iter.as_str())
}

#[cfg(test)]
mod tests {
    use crate::test_format;

    test_format! {
        #[test]
        fn comment_without_space(r"#comment") -> Ok("# comment");
    }

    test_format! {
        #[test]
        fn empty_comment(r"#") -> Ok(source);
    }

    test_format! {
        #[test]
        fn only_space_comment1(r"# ") -> Ok(r"#");
    }

    test_format! {
        #[test]
        fn only_space_comment2(r"#      ") -> Ok(r"#");
    }

    test_format! {
        #[test]
        fn strip_prefix_space(r"#    hello") -> Ok(r"# hello");
    }

    test_format! {
        #[test]
        fn multiline_comment_with_ident(
            r#"
            # NOTE: Tombi preserves spaces at the beginning of a comment line.
            #       This allows for multi-line indentation to be preserved.
            "#
        ) -> Ok(source);
    }
}
