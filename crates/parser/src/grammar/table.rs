use syntax::T;

use crate::{
    grammar::{
        leading_comments, peek_leading_comments, root::NEXT_SECTION, tailing_comment, Grammer,
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
            p.error(crate::Error::ExpectedBraceEnd);
        }

        tailing_comment(p);

        loop {
            let n = peek_leading_comments(p);

            if p.nth_at_ts(n, NEXT_SECTION) {
                break;
            }

            ast::KeyValue::parse(p);
        }

        m.complete(p, TABLE);
    }
}
