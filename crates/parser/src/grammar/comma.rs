use syntax::T;

use crate::grammar::{leading_comments, tailing_comment};

use super::{Grammer, Parser};

impl Grammer for Option<ast::Comma> {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);
        if p.eat(T![,]) {
            tailing_comment(p);
            m.complete(p, T!(,));
        } else {
            m.abandon(p);
        }
    }
}
