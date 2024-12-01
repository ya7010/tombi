use config::TomlVersion;
use syntax::{SyntaxKind::*, T};

use crate::ErrorKind::*;
use crate::{
    parse::{
        begin_dangling_comments, end_dangling_comments, leading_comments, peek_leading_comments,
        tailing_comment, Parse,
    },
    parser::Parser,
};

impl Parse for ast::InlineTable {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        let begin_range = p.current_range();

        assert!(p.at(T!['{']));

        p.eat(T!['{']);

        begin_dangling_comments(p);

        let mut last_comma_range = None;
        loop {
            let n = peek_leading_comments(p);
            if p.nth_at(n, EOF) || p.nth_at(n, T!['}']) {
                break;
            }

            ast::KeyValue::parse(p);

            let n = peek_leading_comments(p);
            if p.nth_at(n, T![,]) {
                last_comma_range = Some(p.nth_range(n));
                ast::Comma::parse(p);
            } else {
                last_comma_range = None;
                if !p.nth_at(n, T!['}']) {
                    p.error(crate::Error::new(ExpectedComma, p.current_range()));
                    p.bump_any();
                }
            }
        }

        end_dangling_comments(p);

        let end_range = p.current_range();

        if !p.eat(T!['}']) {
            p.error(crate::Error::new(ExpectedBraceEnd, p.current_range()));
        }

        if p.toml_version() == TomlVersion::V1_0_0 {
            if begin_range.start().line() != end_range.start().line() {
                p.error(crate::Error::new(
                    InlineTableMustSingleLine,
                    begin_range + end_range,
                ));
            }
            if let Some(comma_range) = last_comma_range {
                p.error(crate::Error::new(
                    ForbiddenInlineTableLastComma,
                    comma_range,
                ));
            }
        }

        tailing_comment(p);

        m.complete(p, INLINE_TABLE);
    }
}
