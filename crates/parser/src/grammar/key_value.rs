use super::{leading_comments, tailing_comment, Grammer, TS_COMMEMT_OR_LINE_END};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

impl Grammer for ast::KeyValue {
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
    use rstest::rstest;
    use syntax::SyntaxError;
    use text::{Column, Line};

    #[rstest]
    #[case::only_key("key1", &[(crate::Error::ExpectedEqual, ((0, 0), (0, 4))), (crate::Error::ExpectedValue, ((0, 0), (0, 4)))])]
    #[case::value_not_found("key1 = # INVALID", &[(crate::Error::ExpectedValue, ((0, 5), (0, 6)))])]
    #[case::invalid_value("key1 = 2024-01-00T", &[(crate::Error::ExpectedValue, ((0, 7), (0, 18)))])]
    #[case::value_not_found_in_multi_key_value(r#"
key1 = 1
key2 = # INVALID
key3 = 3
"#.trim_start(), &[
    (crate::Error::ExpectedValue, ((1, 5), (1, 6))),
])]
    #[case::basic_string_without_begin_quote(r#"
key1 = "str"
key2 = invalid"
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 7), (1, 15)))]
)]
    #[case::basic_string_without_end_quote(r#"
key1 = "str"
key2 = "invalid
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 8), (1, 15)))]
    )]
    #[case::literal_string_without_start_quote(r#"
key1 = 'str'
key2 = invalid'
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 7), (1, 15)))]
    )]
    #[case::literal_string_without_end_quote(r#"
key1 = 'str'
key2 = 'invalid
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 8), (1, 15)))]
    )]
    #[case::without_equal(r#"
key1 "value"
key2 = 1
"#.trim_start(), &[
    (crate::Error::ExpectedEqual, ((0, 5), (0, 12))),
    (crate::Error::ExpectedValue, ((0, 5), (0, 12)))
]
    )]
    #[case::without_equal_on_root_item_with_comment(r#"
key value # comment

[aaa]
key1 = 1
"#.trim_start(), &[
    (crate::Error::ExpectedEqual, ((0, 4), (0, 9))),
    (crate::Error::ExpectedValue, ((0, 4), (0, 9)))
]
    )]
    #[case::without_equal_on_root_item(r#"
key value

[aaa]
key1 = 1
"#.trim_start(), &[
    (crate::Error::ExpectedEqual, ((0, 4), (0, 9))),
    (crate::Error::ExpectedValue, ((0, 4), (0, 9)))
]
    )]
    fn invalid_key_value(
        #[case] source: &str,
        #[case] errors: &[(crate::Error, ((Line, Column), (Line, Column)))],
    ) {
        let p = crate::parse(source);

        assert_eq!(
            p.errors(),
            errors
                .into_iter()
                .map(|(error, range)| SyntaxError::new(*error, (*range).into()))
                .collect::<Vec<_>>()
        );
    }
}
