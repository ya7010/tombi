use crate::{
    grammar::{array_of_table::parse_array_of_table, table::parse_table},
    parser::Parser,
    token_set::TokenSet,
};

use super::{
    begin_dangling_comments, end_dangling_comments, key::KEY_FIRST, key_value::parse_key_value,
    leading_comments, peek_leading_comments, tailing_comment,
};
use syntax::{SyntaxKind::*, T};

const LINE_END: TokenSet = TokenSet::new(&[NEWLINE, COMMENT, EOF]);
pub const NEXT_SECTION: TokenSet = TokenSet::new(&[T!['['], T!("[["), EOF]);

pub fn parse_root(p: &mut Parser<'_>) {
    let m = p.start();

    begin_dangling_comments(p);

    loop {
        let n = peek_leading_comments(p);
        if p.nth_at(n, EOF) {
            break;
        }
        if p.nth_at_ts(n, KEY_FIRST) {
            parse_key_value(p);
        } else if p.nth_at(n, T!("[[")) {
            parse_array_of_table(p);
        } else if p.nth_at(n, T!['[']) {
            parse_table(p);
        } else {
            parse_unknwon_line(p);
        }
    }

    end_dangling_comments(p);

    m.complete(p, ROOT);
}

fn parse_unknwon_line(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    while !p.at_ts(LINE_END) {
        p.bump_any();
    }
    p.error(crate::Error::UnknownLine);

    tailing_comment(p);

    m.complete(p, ERROR);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_begin_dangling_comments() {
        let input = r#"
# begin dangling_comment1
# begin dangling_comment2

# table leading comment1
# table leading comment2
[table]
        "#
        .trim();
        let p = crate::parse(input);

        // dbg!(p.syntax_node());

        assert_eq!(p.errors(), vec![]);
    }
}
