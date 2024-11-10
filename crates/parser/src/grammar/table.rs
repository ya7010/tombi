use syntax::T;

use crate::{
    grammar::{
        invalid_line, leading_comments, peek_leading_comments, tailing_comment, Grammer, LINE_END,
        NEXT_SECTION,
    },
    parser::Parser,
};
use syntax::SyntaxKind::*;

impl Grammer for ast::Table {
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

            if p.nth_at_ts(n, NEXT_SECTION) {
                break;
            }

            ast::KeyValue::parse(p);

            if !p.at_ts(LINE_END) {
                invalid_line(p, crate::Error::ExpectedLineBreak);
            }
        }

        m.complete(p, TABLE);
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use syntax::SyntaxError;
    use text::{Column, Line};

    #[rstest]
    #[case(r#"
[]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedKey, ((0, 0), (0, 1)))]
    #[case(r#"
[aaa.]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedKey, ((0, 4), (0, 5)))]
    #[case(r#"
[aaa.bbb
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedBracketEnd, ((0, 8), (1, 0)))]
    #[case(r#"
[aaa.bbb]
key1 = 1
key2 = 2

[aaa.ccc]
key1 =
key2 = 2

[aaa.ddd]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedValue, ((5, 5), (5, 6)))]
    fn invalid_table(
        #[case] source: &str,
        #[case] error: crate::Error,
        #[case] range: ((Line, Column), (Line, Column)),
    ) {
        let p = crate::parse(source);

        assert_eq!(p.errors(), vec![SyntaxError::new(error, range.into())]);
    }
}
