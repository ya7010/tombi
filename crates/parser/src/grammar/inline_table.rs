use syntax::{SyntaxKind::*, T};

use crate::{
    grammar::{leading_comments, peek_leading_comments, tailing_comment, Grammer},
    parser::Parser,
};

impl Grammer for ast::InlineTable {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!['{']));

        p.eat(T!['{']);

        loop {
            let n = peek_leading_comments(p);
            if p.nth_at(n, EOF) || p.nth_at(n, T!['}']) {
                break;
            }
            ast::KeyValue::parse(p);
            p.eat(T![,]);
            tailing_comment(p);
        }

        if !p.eat(T!['}']) {
            p.error(crate::Error::ExpectedBraceEnd);
        }

        tailing_comment(p);

        m.complete(p, INLINE_TABLE);
    }
}
