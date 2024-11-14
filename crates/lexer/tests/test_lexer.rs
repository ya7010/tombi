use lexer::{tokenize, Token};
use rstest::rstest;
use syntax::{
    SyntaxKind::{self, *},
    T,
};
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
    let source = "{},.[]=";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(T!('{'), (0, 1).into()),
            Token::new(T!('}'), (1, 2).into()),
            Token::new(T!(,), (2, 3).into()),
            Token::new(T!(.), (3, 4).into()),
            Token::new(T!('['), (4, 5).into()),
            Token::new(T!(']'), (5, 6).into()),
            Token::new(T!(=), (6, 7).into())
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

#[rstest]
#[case("'Hello, World!'", (0, 15))]
#[case("'Hello, \\'Taro\\'!'", (0, 18))]
fn literal_string(#[case] source: &str, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(&source).collect();
    assert_eq!(tokens, vec![Token::new(LITERAL_STRING, span.into())]);
}

#[rstest]
#[case("2021-01-01T00:00:00Z", OFFSET_DATE_TIME, (0, 20))]
#[case("2021-01-01T00:00:00+09:00", OFFSET_DATE_TIME, (0, 25))]
#[case("2021-01-01T00:00:00-09:00", OFFSET_DATE_TIME, (0, 25))]
#[case("2021-01-01T00:00:00.123456Z", OFFSET_DATE_TIME, (0, 27))]
#[case("2021-01-01T00:00:00.123456+09:00", OFFSET_DATE_TIME, (0, 32))]
#[case("2021-01-01T00:00:00.123456-09:00", OFFSET_DATE_TIME, (0, 32))]
#[case("2021-01-01 00:00:00", LOCAL_DATE_TIME, (0, 19))]
#[case("2021-01-01 00:00:00.123456", LOCAL_DATE_TIME, (0, 26))]
#[case("2021-01-01T00:00:00", LOCAL_DATE_TIME, (0, 19))]
#[case("2021-01-01T00:00:00.123456", LOCAL_DATE_TIME, (0, 26))]
#[case("2021-01-01", LOCAL_DATE, (0, 10))]
#[case("00:00:00", LOCAL_TIME, (0, 8))]
#[case("00:00:00.123456", LOCAL_TIME, (0, 15))]
fn datetime(#[case] source: &str, #[case] kind: SyntaxKind, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![Token::new(kind, span.into())]);
}

#[rstest]
#[case("true", BOOLEAN, (0, 4))]
#[case("false", BOOLEAN, (0, 5))]
fn boolean(#[case] source: &str, #[case] kind: SyntaxKind, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![Token::new(kind, span.into())]);
}

#[rstest]
#[case("key", (0, 3))]
fn key(#[case] source: &str, #[case] span: (Offset, Offset)) {
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(tokens, vec![Token::new(BARE_KEY, span.into())]);
}
