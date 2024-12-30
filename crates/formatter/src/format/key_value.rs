use ast::AstNode;

use crate::Format;
use std::fmt::Write;

impl Format for ast::KeyValue {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comment in self.leading_comments() {
            comment.fmt(f)?;
        }

        f.write_indent()?;
        self.keys().unwrap().fmt(f)?;

        write!(f, " = ")?;

        f.skip_indent();
        self.value().unwrap().fmt(f)?;

        // NOTE: tailing comment is output by `value.fmt(f)`.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_format;

    test_format! {
        #[test]
        fn bare_key_value1(r#"key = "value""#) -> Ok("key = \"value\"");
    }
    test_format! {
        #[test]
        fn bare_key_value2(r#"key    = "value""#) -> Ok("key = \"value\"");
    }
    test_format! {
        #[test]
        fn dotted_keys_value1(r#"key1.key2.key3 = "value""#) -> Ok(source);
    }
    test_format! {
        #[test]
        fn dotted_keys_value2(r#"site."google.com" = true"#) -> Ok(source);
    }
    test_format! {
        #[test]
        fn key_value_with_comment(
            r#"
            # leading comment1
            # leading comment2
            key = "value"  # tailing comment
            "#
        ) -> Ok(source);
    }
}
