mod array;
mod array_of_table;
mod inline_table;
mod key;
mod key_value;
mod root;
mod table;
mod value;

use root::parse_root;

use crate::output::Output;
use crate::SyntaxKind::*;

pub fn parse(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    parse_root(&mut p);

    let events = p.finish();

    crate::event::process(events)
}

fn begin_dangling_comments(p: &mut crate::parser::Parser<'_>) {
    let mut n = 0;
    let mut comment_count = 0;
    while p.nth_at(n, WHITESPACE) || p.nth_at(n, COMMENT) || p.nth_at(n, NEWLINE) {
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

fn end_dangling_comments(p: &mut crate::parser::Parser<'_>) {
    while p.eat(WHITESPACE) || p.eat(COMMENT) || p.eat(NEWLINE) {}
}

fn peek_leading_comments(p: &mut crate::parser::Parser<'_>) -> usize {
    let mut n = 0;
    while p.nth_at(n, WHITESPACE) || p.nth_at(n, COMMENT) || p.nth_at(n, NEWLINE) {
        n += 1;
    }

    n
}

fn leading_comments(p: &mut crate::parser::Parser<'_>) {
    while p.eat(WHITESPACE) || p.eat(COMMENT) || p.eat(NEWLINE) {}
}

fn tailing_comment(p: &mut crate::parser::Parser<'_>) {
    while p.eat(WHITESPACE) || p.eat(COMMENT) {}
}
