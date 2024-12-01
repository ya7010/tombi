use syntax::{SyntaxKind::*, T};

use crate::ErrorKind::*;
use crate::{
    grammar::{
        begin_dangling_comments, end_dangling_comments, leading_comments, peek_leading_comments,
        tailing_comment, Parse,
    },
    parser::Parser,
};

impl Parse for ast::InlineTable {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        assert!(p.at(T!['{']));

        p.eat(T!['{']);

        begin_dangling_comments(p);

        loop {
            let n = peek_leading_comments(p);
            if p.nth_at(n, EOF) || p.nth_at(n, T!['}']) {
                break;
            }

            ast::KeyValue::parse(p);

            let n = peek_leading_comments(p);
            if p.nth_at(n, T![,]) {
                ast::Comma::parse(p);
            } else if !p.nth_at(n, T!['}']) {
                p.error(crate::Error::new(ExpectedComma, p.current_span()));
                p.bump_any();
            }
        }

        end_dangling_comments(p);

        if !p.eat(T!['}']) {
            p.error(crate::Error::new(ExpectedBraceEnd, p.current_span()));
        }

        tailing_comment(p);

        m.complete(p, INLINE_TABLE);
    }
}
