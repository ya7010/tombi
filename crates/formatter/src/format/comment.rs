use ast::{
    BeginDanglingComment, DanglingComment, EndDanglingComment, LeadingComment, TailingComment,
};

use super::Format;
use std::fmt::Write;

impl Format for ast::Comment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        let comment = self.to_string();
        let mut iter = comment.trim_ascii_end().chars();
        write!(f, "{}", iter.next().unwrap())?;

        if let Some(c) = iter.next() {
            if c != ' ' && c != '\t' {
                write!(f, " ")?;
            }
            write!(f, "{}", c)?;
        }

        write!(f, "{}", iter.as_str())
    }
}

impl Format for DanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.write_indent()?;
        self.as_ref().fmt(f)
    }
}

impl Format for Vec<Vec<DanglingComment>> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for (i, comments) in self.iter().enumerate() {
            assert!(!comments.is_empty());
            if i != 0 {
                write!(f, "{}{}", f.line_ending(), f.line_ending())?;
            }

            for (j, comment) in comments.iter().enumerate() {
                if j != 0 {
                    write!(f, "{}", f.line_ending())?;
                }
                comment.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl Format for BeginDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.write_indent()?;
        self.as_ref().fmt(f)?;
        write!(f, "{}", f.line_ending())
    }
}

impl Format for Vec<Vec<BeginDanglingComment>> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comments in self {
            assert!(!comments.is_empty());

            for comment in comments {
                comment.fmt(f)?;
            }
            write!(f, "{}", f.line_ending())?;
        }

        Ok(())
    }
}

impl Format for EndDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", f.line_ending())?;
        f.write_indent()?;
        self.as_ref().fmt(f)
    }
}

impl Format for Vec<Vec<EndDanglingComment>> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for (i, comments) in self.iter().enumerate() {
            if i != 0 {
                write!(f, "{}", f.line_ending())?;
            }

            for comment in comments {
                comment.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl Format for LeadingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.write_indent()?;
        self.as_ref().fmt(f)?;
        write!(f, "{}", f.line_ending())
    }
}

impl Format for TailingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", f.tailing_comment_space())?;
        self.as_ref().fmt(f)
    }
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
}
