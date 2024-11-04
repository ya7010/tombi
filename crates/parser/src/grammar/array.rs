use syntax::T;

use crate::{
    grammar::{leading_comments, peek_leading_comments, tailing_comment, Grammer},
    parser::Parser,
};
use syntax::SyntaxKind::*;

impl Grammer for ast::Array {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!['[']));

        p.eat(T!['[']);

        loop {
            let n = peek_leading_comments(p);
            if p.nth_at(n, EOF) || p.nth_at(n, T![']']) {
                break;
            }

            ast::Value::parse(p);
            while p.eat(WHITESPACE) || p.eat(NEWLINE) || p.eat(COMMENT) {}
            p.eat(T![,]);
            tailing_comment(p);
        }

        if !p.eat(T![']']) {
            p.error(crate::Error::ExpectedBracketEnd);
        }

        tailing_comment(p);

        m.complete(p, ARRAY);
    }
}
