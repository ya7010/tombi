use super::comment::{LeadingComment, TailingComment};
use crate::Format;
use std::fmt::Write;

impl Format for ast::Table {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        let header = self.header().unwrap();
        let key_values = self.key_values().collect::<Vec<_>>();

        for comment in self.header_leading_comments() {
            LeadingComment(comment).fmt(f)?;
        }

        write!(f, "[{header}]")?;

        if let Some(comment) = self.header_tailing_comment() {
            TailingComment(comment).fmt(f)?;
        }

        key_values
            .iter()
            .map(|kv| {
                write!(f, "\n")?;
                kv.fmt(f)
            })
            .collect::<Result<(), std::fmt::Error>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use ast::AstNode;
    use rstest::rstest;

    #[rstest]
    #[case(r#"[package]"#)]
    #[case(r#"[dependencies."unicase"]"#)]
    #[case(r#"[dependencies.unicase]"#)]
    fn table_only_header(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
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

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case(
        r#"
# header leading comment1
# header leading comment2
[header]  # header tailing comment
# key value leading comment1
# key value leading comment2
key = "value"  # key tailing comment
"#.trim()
    )]
    #[case(
        r#"
  # header leading comment1
 # header leading comment2
[header]# header tailing comment

  # key value leading comment1
 # key value leading comment2
key = "value" # key tailing comment
"#.trim()
    )]
    fn table_comment(#[case] source: &str) {
        let expected = r#"
# header leading comment1
# header leading comment2
[header]  # header tailing comment
# key value leading comment1
# key value leading comment2
key = "value"  # key tailing comment
"#
        .trim();

        let result = crate::format(&source);
        assert_matches!(result, Ok(_));
        assert_eq!(result.unwrap(), expected);
    }
}
