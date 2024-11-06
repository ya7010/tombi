use ast::AstNode;

use crate::Format;
use std::fmt::Write;

use super::comment::{BeginDanglingComment, EndDanglingComment, LeadingComment, TailingComment};

impl Format for ast::Array {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.has_tailing_comma_after_last_element() || self.has_inner_comments() {
            format_multiline_array(self, f)
        } else {
            format_singleline_array(self, f)
        }
    }
}

fn format_multiline_array(
    array: &ast::Array,
    f: &mut crate::Formatter,
) -> Result<(), std::fmt::Error> {
    for comment in array.leading_comments() {
        LeadingComment(comment).fmt(f)?;
    }

    write!(f, "[\n")?;

    f.inc_ident();
    f.inc_ident();

    let inner_begin_dangling_comments = array.inner_begin_dangling_comments();
    if inner_begin_dangling_comments.len() > 0 {
        for comment in inner_begin_dangling_comments {
            BeginDanglingComment(comment).fmt(f)?;
        }
        write!(f, "\n")?;
    }

    for (i, (value, comma)) in array.values_with_comma().into_iter().enumerate() {
        // value format
        {
            if i > 0 {
                write!(f, "\n")?;
            }
            value.fmt(f)?;
        }

        // comma format
        {
            let (comma_leading_comments, comma_tailing_comment) = match comma {
                Some(comma) => {
                    (comma.leading_comments(), comma.tailing_comment())
                }
                None => (vec![], None),
            };
    
            if comma_leading_comments.len() > 0 {
                write!(f, "\n")?;
                for comment in comma_leading_comments {
                    LeadingComment(comment).fmt(f)?;
                }
                write!(f, "{},", f.ident())?;
            } else {
                if value.tailing_comment().is_some() {
                    write!(f, "\n{},", f.ident())?;
                } else {
                    write!(f, ",")?;
                }
            }
    
            if let Some(comment) = comma_tailing_comment {
                TailingComment(comment).fmt(f)?;
            }
        }
    }

    let inner_end_dangling_comments = array.inner_end_dangling_comments();
    if inner_end_dangling_comments.len() > 0 {
        write!(f, "\n")?;
        for comment in inner_end_dangling_comments {
            EndDanglingComment(comment).fmt(f)?;
        }
    }

    f.dec_ident();

    write!(f, "\n{}]", f.ident())?;

    if let Some(comment) = array.tailing_comment() {
        TailingComment(comment).fmt(f)?;
    }

    f.dec_ident();

    Ok(())
}

fn format_singleline_array(
    array: &ast::Array,
    f: &mut crate::Formatter,
) -> Result<(), std::fmt::Error> {
    write!(f, "[{}", f.defs().singleline_array_bracket_inner_space())?;

    let values = array.values().collect::<Vec<_>>();
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            write!(f, ",{}", f.defs().singleline_array_space_after_comma())?;
        }
        value.fmt(f)?;
    }

    write!(f, "{}]", f.defs().singleline_array_bracket_inner_space())?;

    if let Some(comment) = array.tailing_comment() {
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
    #[case(r#"integers = [ 1, 2, 3 ]"#, r#"integers = [1, 2, 3]"#)]
    #[case(
        r#"colors = [ "red", "yellow", "green" ]"#,
        r#"colors = ["red", "yellow", "green"]"#
    )]
    #[case(
        r#"nested_arrays_of_ints = [ [ 1, 2 ], [ 3, 4, 5 ] ]"#,
        r#"nested_arrays_of_ints = [[1, 2], [3, 4, 5]]"#
    )]
    #[case(
        r#"nested_mixed_array = [ [ 1, 2 ], [ "a", "b", "c" ] ]"#,
        r#"nested_mixed_array = [[1, 2], ["a", "b", "c"]]"#
    )]
    #[case(
        r#"string_array = [ "all", 'strings', """are the same""", '''type''' ]"#,
        r#"string_array = ["all", 'strings', """are the same""", '''type''']"#
    )]
    fn inline_array(#[case] source: &str, #[case] expected: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, expected);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("[1, 2, 3,]", true)]
    #[case("[1, 2, 3]", false)]
    fn has_tailing_comma_after_last_element(#[case] source: &str, #[case] expected: bool) {
        let p = parser::parse_as::<ast::Array>(source);
        assert_eq!(p.errors(), []);

        let ast = ast::Array::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.has_tailing_comma_after_last_element(), expected);
    }

    #[rstest]
    #[case(
        r#"
# array leading comment1
# array leading comment2
array = [

    # inner array begin dangling comment1
    # inner array begin dangling comment2

    # item1 leading comment1
    # item1 leading comment2
    1 # item1 trailing comment
    , # item1 comma tailing comment
    2 # item2 trailing comment
    # item2 comma leading comment1
    # item2 comma leading comment2
    , # item2 comma tailing comment
    # item3 leading comment1
    # item3 leading comment2
    3 # item3 trailing comment
    # array end dangling comment1

    # array end dangling comment2

] # array tailing comment
"#.trim(),
r#"
# array leading comment1
# array leading comment2
array = [
    # inner array begin dangling comment1
    # inner array begin dangling comment2

    # item1 leading comment1
    # item1 leading comment2
    1  # item1 trailing comment
    ,  # item1 comma tailing comment
    2  # item2 trailing comment
    # item2 comma leading comment1
    # item2 comma leading comment2
    ,  # item2 comma tailing comment
    # item3 leading comment1
    # item3 leading comment2
    3  # item3 trailing comment
    ,

    # array end dangling comment1
    # array end dangling comment2
  ]  # array tailing comment
"#.trim()
    )]
    fn multiline_array_with_comment(#[case] source: &str, #[case] expected: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();

        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, expected);
        assert_eq!(p.errors(), []);
    }
}
