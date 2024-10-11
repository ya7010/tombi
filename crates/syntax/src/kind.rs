#[doc = r" The kind of syntax node, e.g. `WHITESPACE`, `COMMENT`, or `TABLE`."]
#[derive(logos::Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
#[logos(error = crate::Error)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    ROOT = 0,

    #[regex(r"[ \t]+")]
    WHITESPACE,

    #[regex(r"(\n|\r\n)+")]
    NEWLINE,

    #[token(".")]
    PERIOD,

    #[token(",")]
    COMMA,

    #[token("=")]
    EQUAL,

    #[token("[")]
    BRACKET_START,

    #[token("]")]
    BRACKET_END,

    #[token("{")]
    BRACE_START,

    #[token("}")]
    BRACE_END,

    #[regex(r"[A-Za-z0-9_-]+", priority = 2)]
    BARE_KEY,

    #[regex(r#"""#, |lex| lex_single_line_string(lex, '"'))]
    BASIC_STRING,

    #[regex(r#"""""#, |lex| lex_multi_line_string(lex, '"'))]
    MULTI_LINE_BASIC_STRING,

    #[regex(r#"'"#, |lex| lex_single_line_string(lex, '\''))]
    LITERAL_STRING,

    #[regex(r"'''", |lex| lex_multi_line_string(lex, '\''))]
    MULTI_LINE_LITERAL_STRING,

    #[regex(r"[+-]?[0-9_]+", priority = 4)]
    INTEGER,

    #[regex(r"0x[0-9A-Fa-f_]+")]
    INTEGER_HEX,

    #[regex(r"0o[0-7_]+")]
    INTEGER_OCT,

    #[regex(r"0b(0|1|_)+")]
    INTEGER_BIN,

    #[regex(r"[-+]?([0-9_]+(\.[0-9_]+)?([eE][+-]?[0-9_]+)?|nan|inf)", priority = 3)]
    FLOAT,

    #[regex(r"true|false")]
    BOOLEAN,

    #[regex(r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?(?:[Zz]|[+-][01]\d:[0-5]\d)"#)]
    OFFSET_DATE_TIME,

    #[regex(r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?"#)]
    LOCAL_DATE_TIME,

    #[regex(r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)"#)]
    LOCAL_DATE,

    #[regex(r#"(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?"#)]
    LOCAL_TIME,

    #[regex(r"#[^\n\r]*")]
    COMMENT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    #[inline]
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
    use super::*;
    use logos::Logos;

    #[test]
    fn bare_key() {
        let mut lex = SyntaxKind::lexer("test");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
    }

    #[test]
    fn key_value() {
        let mut lex = SyntaxKind::lexer("key = 'value'");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LITERAL_STRING)));
    }

    #[test]
    fn inline_table() {
        let mut lex = SyntaxKind::lexer("key1 = { key2 = 'value' }");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LITERAL_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_END)));
    }

    #[test]
    fn invalid_source() {
        let mut lex = SyntaxKind::lexer("key1 = { key2 = 'value");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Err(crate::Error::InvalidToken)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
    }
}
