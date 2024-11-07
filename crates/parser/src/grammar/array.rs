use syntax::T;

use crate::{
    grammar::{
        begin_dangling_comments, end_dangling_comments, leading_comments, peek_leading_comments,
        tailing_comment, Grammer,
    },
    parser::Parser,
};
use syntax::SyntaxKind::*;

impl Grammer for ast::Array {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!['[']));

        p.eat(T!['[']);

        begin_dangling_comments(p);

        loop {
            let n = peek_leading_comments(p);
            if p.nth_at(n, EOF) || p.nth_at(n, T![']']) {
                break;
            }

            ast::Value::parse(p);

            Option::<ast::Comma>::parse(p);
        }

        end_dangling_comments(p);

        if !p.eat(T![']']) {
            p.error(crate::Error::ExpectedBracketEnd);
        }

        tailing_comment(p);

        m.complete(p, ARRAY);
    }
}
