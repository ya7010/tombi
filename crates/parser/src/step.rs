use syntax::SyntaxKind;

#[derive(Debug)]
pub enum StrStep<'a> {
    Token { kind: SyntaxKind, text: &'a str },
    Enter { kind: SyntaxKind, pos: usize },
    Exit,
    Error { error: crate::Error, pos: usize },
}
