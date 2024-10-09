#[doc = r" The kind of syntax node, e.g. `WHITESPACE`, `COMMENT`, or `TABLE`."]
#[derive(logos::Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
#[logos(skip r"[ \t]+")]
pub enum SyntaxKind {
    #[regex(r"(\n|\r\n)+")]
    Newline,

    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token("=")]
    Equal,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[regex(r"[A-Za-z0-9_-]+", priority = 2)]
    BareKey,

    #[regex(r#"""#, |lex| lex_single_line_string(lex, '"'))]
    BasicString,

    #[regex(r#"""""#, |lex| lex_multi_line_string(lex, '"'))]
    MultiLineBasicString,

    #[regex(r#"'"#, |lex| lex_single_line_string(lex, '\''))]
    LiteralString,

    #[regex(r"'''", |lex| lex_multi_line_string(lex, '\''))]
    MultiLineLiteralString,

    #[regex(r"#[^\n\r]*")]
    Comment,

    ROOT, // root node
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

fn lex_single_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == quote {
            lex.bump(remainder[0..total_len].as_bytes().len());
            return true;
        }
    }
    false
}

fn lex_multi_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
    let remainder: &str = lex.remainder();

    let mut total_len = 0;
    let mut quote_count = 0;
    let mut escaped = false;

    // As the string can contain ",
    // we can end up with more than 3 "-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    for c in remainder.chars() {
        if quotes_found {
            if c != quote {
                if quote_count >= 6 {
                    return false;
                }

                lex.bump(remainder[0..total_len].as_bytes().len());
                return true;
            } else {
                quote_count += 1;
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = true;
            continue;
        }

        if c == quote && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }

        escaped = false;
    }

    // End of input
    if quotes_found {
        if quote_count >= 6 {
            return false;
        }

        lex.bump(remainder[0..total_len].as_bytes().len());
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::SyntaxKind;
    use logos::Logos;

    #[test]
    fn bare_key() {
        let mut lex = SyntaxKind::lexer("test");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BareKey)));
    }

    #[test]
    fn key_value() {
        let mut lex = SyntaxKind::lexer("key = 'value'");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BareKey)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::Equal)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LiteralString)));
    }

    #[test]
    fn inline_table() {
        let mut lex = SyntaxKind::lexer("key1 = { key2 = 'value' }");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BareKey)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::Equal)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BareKey)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::Equal)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LiteralString)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BraceClose)));
    }
}
