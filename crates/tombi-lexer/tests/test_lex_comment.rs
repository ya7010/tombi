use itertools::Itertools;
use tombi_lexer::{tokenize_comments, Token};
use tombi_syntax::SyntaxKind::*;

macro_rules! test_lex_comments {
    {#[test]fn $name:ident($source:expr) -> [
        $(Token($kind:expr, $text:literal),)*
    ];} => {
        #[test]
        fn $name() {
            tombi_test_lib::init_tracing();

            let tokens = tokenize_comments($source).collect_vec();
            let (expected, _) = [
                $(
                    ($kind, $text),
                )*
            ]
            .into_iter()
            .fold((vec![], (0, tombi_text::Position::MIN)), |(mut acc, (start_offset, start_position)), (kind, text)| {
                let text: &str = text;
                let end_offset = start_offset + (text.len() as u32);
                let end_position = start_position + tombi_text::RelativePosition::of(text);
                acc.push(
                    Ok(
                        Token::new(
                            kind,
                            (
                                (start_offset, end_offset).into(),
                                (start_position, end_position).into()
                            )
                        )
                    )
                );
                (acc, (end_offset, end_position))
            });
            pretty_assertions::assert_eq!(tokens, expected);
        }
    };
}

test_lex_comments! {
    #[test]
    fn empty_source("") -> [];
}

test_lex_comments! {
    #[test]
    fn comment_line_break("# This is a comment\n") -> [
        Token(COMMENT, "# This is a comment"),
        Token(LINE_BREAK, "\n"),
    ];
}

test_lex_comments! {
    #[test]
    fn comment_line_break_crlf("# This is a comment\r\n") -> [
        Token(COMMENT, "# This is a comment"),
        Token(LINE_BREAK, "\r\n"),
    ];
}

test_lex_comments! {
    #[test]
    fn whitespace_comment_line_break("   # This is a comment\n") -> [
        Token(WHITESPACE, "   "),
        Token(COMMENT, "# This is a comment"),
        Token(LINE_BREAK, "\n"),
    ];
}

test_lex_comments! {
    #[test]
    fn whitespace_comment_line_break_crlf("   # This is a comment\r\n") -> [
        Token(WHITESPACE, "   "),
        Token(COMMENT, "# This is a comment"),
        Token(LINE_BREAK, "\r\n"),
    ];
}

test_lex_comments! {
    #[test]
    fn comment_whitespace_line_break("# This is a comment  \n") -> [
        Token(COMMENT, "# This is a comment  "),
        Token(LINE_BREAK, "\n"),
    ];
}

test_lex_comments! {
    #[test]
    fn comments("# This is a comment\n# This is another comment") -> [
        Token(COMMENT, "# This is a comment"),
        Token(LINE_BREAK, "\n"),
        Token(COMMENT, "# This is another comment"),
    ];
}

test_lex_comments! {
    #[test]
    fn tokens("{") -> [];
}

test_lex_comments! {
    #[test]
    fn key_value_float_dot_key("3.14159") -> [];
}
