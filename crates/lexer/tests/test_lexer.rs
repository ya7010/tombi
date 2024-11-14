use lexer::{tokenize, Token};
use rstest::rstest;
use syntax::{SyntaxKind::*, T};
use text::Offset;

#[test]
fn empty_source() {
    let source = "";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![]);
}

#[test]
fn only_comment() {
    let source = "# This is a comment";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![Token::new(COMMENT, (0, 19).into())]);
}

#[test]
fn comment_line_break() {
    let source = "# This is a comment\n";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(COMMENT, (0, 19).into()),
            Token::new(LINE_BREAK, (19, 20).into())
        ]
    );
}

#[test]
fn comment_line_break_crlf() {
    let source = "# This is a comment\r\n";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(COMMENT, (0, 19).into()),
            Token::new(LINE_BREAK, (19, 21).into())
        ]
    );
}

#[test]
fn whitespace_comment_line_break_crlf() {
    let source = "   # This is a comment\r\n";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(WHITESPACE, (0, 3).into()),
            Token::new(COMMENT, (3, 22).into()),
            Token::new(LINE_BREAK, (22, 24).into())
        ]
    );
}

#[test]
fn tokens() {
    let source = "{},[]=";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(T!('{'), (0, 1).into()),
            Token::new(T!('}'), (1, 2).into()),
            Token::new(T!(,), (2, 3).into()),
            Token::new(T!('['), (3, 4).into()),
            Token::new(T!(']'), (4, 5).into()),
            Token::new(T!(=), (5, 6).into())
        ]
    );
}

#[rstest]
#[case(r#""Hello, World!""#, (0, 15))]
#[case(r#""Hello, \"Taro\"!""#, (0, 18))]
fn basic_string(#[case] source: &str, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![Token::new(BASIC_STRING, span.into())]);
}

#[rstest]
#[case(r#""""aaaa""""#, (0, 10))]
#[case(r#"
"""
aaaa
"""
"#.trim(), (0, 12))]
fn multi_line_basic_string(#[case] source: &str, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(source).collect();

    assert_eq!(
        tokens,
        vec![Token::new(MULTI_LINE_BASIC_STRING, span.into())]
    );
}
