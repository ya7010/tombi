use syntax::T;

use crate::{
    grammar::{
        invalid_line, leading_comments, peek_leading_comments, tailing_comment, Parse, TS_LINE_END,
        TS_NEXT_SECTION,
    },
    parser::Parser,
};
use syntax::SyntaxKind::*;

impl Parse for ast::Table {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!['[']));

        p.eat(T!['[']);

        ast::Keys::parse(p);

        if !p.eat(T![']']) {
            invalid_line(p, crate::Error::ExpectedBracketEnd);
        }

        tailing_comment(p);

        loop {
            let n = peek_leading_comments(p);

            if p.nth_at_ts(n, TS_NEXT_SECTION) {
                break;
            }

            ast::KeyValue::parse(p);

            if !p.at_ts(TS_LINE_END) {
                invalid_line(p, crate::Error::ExpectedLineBreakOrComment);
            }
        }

        m.complete(p, TABLE);
    }
}

#[cfg(test)]
mod test {
    use crate::test_parser;
    use crate::Error::*;

    test_parser! {
        #[test]
        fn without_header_keys(
            r#"
                []
                key1 = 1
                key2 = 2
                "#
        ) -> Err([
            SyntaxError(ExpectedKey, 0:0..0:1),
        ])
    }

    test_parser! {
        #[test]
        fn without_last_dot_key(
            r#"
            [aaa.]
            key1 = 1
            key2 = 2
            "#
        ) -> Err([
            SyntaxError(ExpectedKey, 0:4..0:5),
        ])
    }

    test_parser! {
        #[test]
        fn without_last_bracket(
            r#"
            [aaa.bbb
            key1 = 1
            key2 = 2
            "#
        ) -> Err([
            SyntaxError(ExpectedBracketEnd, 0:8..1:0),
        ])
    }

    test_parser! {
        #[test]
        fn without_value(
            r#"
            [aaa.bbb]
            key1 = 1
            key2 = 2

            [aaa.ccc]
            key1 =
            key2 = 2

            [aaa.ddd]
            key1 = 1
            key2 = 2
            "#
        ) -> Err([
            SyntaxError(ExpectedValue, 5:5..5:6),
        ])
    }

    test_parser! {
        #[test]
        fn invalid_key_value_tailing_comment(
            r#"
            [aaa.bbb]
            key1 = 1 INVALID COMMENT
            key2 = 2
            "#
        ) -> Err([
            SyntaxError(ExpectedLineBreakOrComment, 1:9..1:16),
        ])
    }
}
