use lexer::{tokenize, Token};
use syntax::SyntaxKind;

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
    assert_eq!(
        tokens,
        vec![Token::new(SyntaxKind::COMMENT, (0, 19).into())]
    );
}

#[test]
fn comment_line_break() {
    let source = "# This is a comment\n";
    let tokens: Vec<Token> = tokenize(source).collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(SyntaxKind::COMMENT, (0, 19).into()),
            Token::new(SyntaxKind::LINE_BREAK, (19, 20).into())
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
            Token::new(SyntaxKind::COMMENT, (0, 19).into()),
            Token::new(SyntaxKind::LINE_BREAK, (19, 21).into())
        ]
    );
}
