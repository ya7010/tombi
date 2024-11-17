use lexer::{tokenize, Token};
use rstest::rstest;
use syntax::{
    SyntaxKind::{self, *},
    T,
};

#[test]
fn empty_source() {
    let source = "";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![]);
}

#[test]
fn only_comment() {
    let source = "# This is a comment";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(COMMENT, (0, 19).into()))]);
}

#[test]
fn comment_line_break() {
    let source = "# This is a comment\n";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Ok(Token::new(COMMENT, (0, 19).into())),
            Ok(Token::new(LINE_BREAK, (19, 20).into()))
        ]
    );
}

#[test]
fn comment_line_break_crlf() {
    let source = "# This is a comment\r\n";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Ok(Token::new(COMMENT, (0, 19).into())),
            Ok(Token::new(LINE_BREAK, (19, 21).into()))
        ]
    );
}

#[test]
fn whitespace_comment_line_break_crlf() {
    let source = "   # This is a comment\r\n";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Ok(Token::new(WHITESPACE, (0, 3).into())),
            Ok(Token::new(COMMENT, (3, 22).into())),
            Ok(Token::new(LINE_BREAK, (22, 24).into()))
        ]
    );
}

#[test]
fn tokens() {
    let source = "{},.[]=";
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Ok(Token::new(T!('{'), (0, 1).into())),
            Ok(Token::new(T!('}'), (1, 2).into())),
            Ok(Token::new(T!(,), (2, 3).into())),
            Ok(Token::new(T!(.), (3, 4).into())),
            Ok(Token::new(T!('['), (4, 5).into())),
            Ok(Token::new(T!(']'), (5, 6).into())),
            Ok(Token::new(T!(=), (6, 7).into()))
        ]
    );
}

#[rstest]
#[case(r#""Hello, World!""#, (0, 15))]
#[case(r#""Hello, \"Taro\"!""#, (0, 18))]
fn basic_string(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(BASIC_STRING, span.into()))]);
}

#[rstest]
#[case(r#""""aaaa""""#, (0, 10))]
#[case(r#"
"""
aaaa
"""
"#.trim(), (0, 12))]
fn multi_line_basic_string(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();

    assert_eq!(
        tokens,
        vec![Ok(Token::new(MULTI_LINE_BASIC_STRING, span.into()))]
    );
}

#[rstest]
#[case("'Hello, World!'", (0, 15))]
#[case("'Hello, \\'Taro\\'!'", (0, 18))]
fn literal_string(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(LITERAL_STRING, span.into()))]);
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
fn datetime(#[case] source: &str, #[case] kind: SyntaxKind, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(kind, span.into()))]);
}

#[rstest]
#[case("true", BOOLEAN, (0, 4))]
#[case("false", BOOLEAN, (0, 5))]
fn boolean(#[case] source: &str, #[case] kind: SyntaxKind, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(kind, span.into()))]);
}

#[rstest]
#[case("key", (0, 3))]
#[case("_1234567890", (0, 11))]
fn key(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(BARE_KEY, span.into()))]);
}

#[rstest]
#[case("0", (0, 1))]
#[case("1", (0, 1))]
#[case("1234567890", (0, 10))]
#[case("+1234567890", (0, 11))]
#[case("-1234567890", (0, 11))]
#[case("1_234_567_890", (0, 13))]
#[case("+1_234_567_890", (0, 14))]
#[case("-1_234_567_890", (0, 14))]
fn integer_dec(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(INTEGER_DEC, span.into()))]);
}

#[rstest]
#[case("+_1234567890", (0, 12))]
#[case("-_1234567890", (0, 12))]
fn invalid_integer_dec(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(INVALID_TOKEN, span.into()))]);
}

#[rstest]
#[case("0b0", (0, 3))]
#[case("0b1", (0, 3))]
#[case("0b01", (0, 4))]
#[case("0b10", (0, 4))]
#[case("0b101010", (0, 8))]
#[case("0b_1010_10", (0, 10))]
#[case("0b10_101_010", (0, 12))]
fn integer_bin(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(INTEGER_BIN, span.into()))]);
}

#[rstest]
#[case("0o0", (0, 3))]
#[case("0o1", (0, 3))]
#[case("0o01", (0, 4))]
#[case("0o10", (0, 4))]
#[case("0o1234567", (0, 9))]
#[case("0o_1234_567", (0, 11))]
#[case("0o12_34_567", (0, 11))]
fn integer_oct(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(INTEGER_OCT, span.into()))]);
}

#[rstest]
#[case("0x0", (0, 3))]
#[case("0x1", (0, 3))]
#[case("0x01", (0, 4))]
#[case("0x10", (0, 4))]
#[case("0x1234567890abcdef", (0, 18))]
#[case("0x_1234_5678_90ab_cdef", (0, 22))]
#[case("0x12_34_5678_90ab_cdef", (0, 22))]
fn integer_hex(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(INTEGER_HEX, span.into()))]);
}

#[rstest]
#[case("+1.0", (0, 4))]
#[case("3.1415", (0, 6))]
#[case("-0.01", (0, 5))]
#[case("5e+22", (0, 5))]
#[case("1e06", (0, 4))]
#[case("-2E-2", (0, 5))]
#[case("6.626e-34", (0, 9))]
fn float(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(FLOAT, span.into()))]);
}

#[rstest]
#[case("inf", (0, 3))]
#[case("nan", (0, 3))]
#[case("+inf", (0, 4))]
#[case("+nan", (0, 4))]
#[case("-inf", (0, 4))]
#[case("-nan", (0, 4))]
fn special_float(#[case] source: &str, #[case] span: impl Into<text::Span>) {
    let tokens = tokenize(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Ok(Token::new(FLOAT, span.into()))]);
}
