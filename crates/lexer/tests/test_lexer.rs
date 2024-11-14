use lexer::{tokenize, Token};
use syntax::SyntaxKind::*;

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
