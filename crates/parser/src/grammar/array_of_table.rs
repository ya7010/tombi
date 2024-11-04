use syntax::T;

use super::Grammer;
use crate::{
    grammar::{leading_comments, peek_leading_comments, root::NEXT_SECTION, tailing_comment},
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
            p.error(crate::Error::ExpectedDoubleBracetEnd);
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
