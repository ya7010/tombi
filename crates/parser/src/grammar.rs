mod array;
mod array_of_table;
mod comma;
mod inline_table;
mod key;
mod key_value;
mod root;
mod table;
mod value;

use crate::{output::Output, parser::Parser};
use support::*;

pub fn parse<G: Grammer>(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    G::parse(&mut p);

    let events = p.finish();

    crate::event::process(events)
}

pub(crate) trait Grammer {
    fn parse(p: &mut Parser<'_>);
}

mod support {
    use crate::{token_set::TokenSet, SyntaxKind::*};

    const DANGLING_COMMENTS_KINDS: TokenSet = TokenSet::new(&[COMMENT, NEWLINE, WHITESPACE]);
    const LEADING_COMMENTS_KINDS: TokenSet = TokenSet::new(&[COMMENT, NEWLINE, WHITESPACE]);
    const TAILING_COMMENT_KINDS: TokenSet = TokenSet::new(&[COMMENT, WHITESPACE]);

    pub fn begin_dangling_comments(p: &mut crate::parser::Parser<'_>) {
        let mut n = 0;
        let mut comment_count = 0;
        while p.nth_at_ts(n, DANGLING_COMMENTS_KINDS) {
            let kind = p.nth(n);
            match kind {
                COMMENT => {
                    comment_count += 1;
                }
                NEWLINE => {
                    if p.nth_at(n + 1, NEWLINE) {
                        if comment_count > 0 {
                            (0..=n).for_each(|_| p.bump_any());
                            while p.eat(NEWLINE) || p.eat(WHITESPACE) {}
                            break;
                        }
                        n += 1;
                    }
                }
                _ => {}
            }
            n += 1;
        }
    }

    pub fn end_dangling_comments(p: &mut crate::parser::Parser<'_>) {
        while p.eat_ts(DANGLING_COMMENTS_KINDS) {}
    }

    pub fn peek_leading_comments(p: &mut crate::parser::Parser<'_>) -> usize {
        let mut n = 0;
        while p.nth_at_ts(n, LEADING_COMMENTS_KINDS) {
            n += 1;
        }

        n
    }

    #[inline]
    pub fn leading_comments(p: &mut crate::parser::Parser<'_>) {
        while p.eat_ts(LEADING_COMMENTS_KINDS) {}
    }

    #[inline]
    pub fn tailing_comment(p: &mut crate::parser::Parser<'_>) {
        while p.eat_ts(TAILING_COMMENT_KINDS) {}
    }
}
