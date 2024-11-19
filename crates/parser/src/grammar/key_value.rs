use super::{leading_comments, tailing_comment, Parse, TS_COMMEMT_OR_LINE_END};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

impl Parse for ast::KeyValue {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        ast::Keys::parse(p);

        if !p.eat(T![=]) {
            if !p.at(LINE_BREAK) {
                p.bump_remap(INVALID_TOKEN);
            }
            p.error(crate::Error::ExpectedEqual);
        }

        if p.at_ts(TS_COMMEMT_OR_LINE_END) {
            p.error(crate::Error::ExpectedValue);
        } else {
            ast::Value::parse(p);
        }

        tailing_comment(p);

        m.complete(p, KEY_VALUE);
    }
}

#[cfg(test)]
mod test {
    use crate::test_parser;
    use crate::Error::*;

    test_parser! {
        #[test]
        fn only_key("key1") -> Err([
            SyntaxError(ExpectedEqual, 0:0..0:4),
            SyntaxError(ExpectedValue, 0:0..0:4),
        ])
    }

    test_parser! {
        #[test]
        fn value_not_found("key1 = # INVALID") -> Err([
            SyntaxError(ExpectedValue, 0:5..0:6),
        ])
    }

    test_parser! {
        #[test]
        fn invalid_value("key1 = 2024-01-00T") -> Err([
            SyntaxError(ExpectedValue, 0:7..0:18),
        ])
    }

    test_parser! {
        #[test]
        fn value_not_found_in_multi_key_value(
            r#"
            key1 = 1
            key2 = # INVALID
            key3 = 3
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 1:5..1:6),
        ])
    }

    test_parser! {
        #[test]
        fn basic_string_without_begin_quote(
            r#"
            key1 = "str"
            key2 = invalid"
            key3 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 1:7..1:15),
        ])
    }

    test_parser! {
        #[test]
        fn basic_string_without_end_quote(
            r#"
            key1 = "str"
            key2 = "invalid
            key3 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 1:7..1:15),
        ])
    }

    test_parser! {
        #[test]
        fn literal_string_without_start_quote(
            r#"
            key1 = 'str'
            key2 = invalid'
            key3 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 1:7..1:15),
        ])
    }

    test_parser! {
        #[test]
        fn literal_string_without_end_quote(
            r#"
            key1 = 'str'
            key2 = 'invalid
            key3 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 1:7..1:15),
        ])
    }

    test_parser! {
        #[test]
        fn without_equal(
            r#"
            key1 "value"
            key2 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedEqual, 0:5..0:12),
            SyntaxError(ExpectedValue, 0:5..0:12),
        ])
    }

    test_parser! {
        #[test]
        fn without_equal_on_root_item_with_comment(
            r#"
            key value # comment

            [aaa]
            key1 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedEqual, 0:4..0:9),
            SyntaxError(ExpectedValue, 0:4..0:9),
        ])
    }

    test_parser! {
        #[test]
        fn without_equal_on_root_item(
            r#"
            key value

            [aaa]
            key1 = 1
            "#
        ) -> Err([
            SyntaxError(ExpectedEqual, 0:4..0:9),
            SyntaxError(ExpectedValue, 0:4..0:9),
        ])
    }
}
