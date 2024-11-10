use syntax::T;

use super::Grammer;
use crate::{
    grammar::{
        invalid_line, leading_comments, peek_leading_comments, tailing_comment, NEXT_SECTION,
    },
    parser::Parser,
};
use syntax::SyntaxKind::*;

impl Grammer for ast::ArrayOfTable {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!("[[")));

        p.eat(T!("[["));

        ast::Keys::parse(p);

        if !p.eat(T!("]]")) {
            invalid_line(p);
            p.error(crate::Error::ExpectedDoubleBracketEnd);
        }

        tailing_comment(p);

        loop {
            let n = peek_leading_comments(p);

            if p.nth_at_ts(n, NEXT_SECTION) {
                break;
            }
            ast::KeyValue::parse(p);
        }

        m.complete(p, ARRAY_OF_TABLE);
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use syntax::SyntaxError;
    use text::{Column, Line};

    #[rstest]
    #[case(r#"
[[]]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedKey, ((0, 0), (0, 2)))]
    #[case(r#"
[[aaa.]]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedKey, ((0, 5), (0, 6)))]
    #[case(r#"
[[aaa.bbb
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedDoubleBracketEnd, ((0, 6), (0, 9)))]
    #[case(r#"
[[aaa.bbb]
key1 = 1
key2 = 2
"#.trim_start(), crate::Error::ExpectedDoubleBracketEnd, ((0, 9), (0, 10)))]
    fn invalid_array_of_table(
        #[case] source: &str,
        #[case] error: crate::Error,
        #[case] range: ((Line, Column), (Line, Column)),
    ) {
        let p = crate::parse(source);

        dbg!(p.syntax_node());

        assert_eq!(p.errors(), vec![SyntaxError::new(error, range.into())]);
    }
}
