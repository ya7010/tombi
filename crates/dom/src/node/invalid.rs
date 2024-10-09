#[derive(Debug, Clone)]
pub struct InvalidNode<'a> {
    pub syntax: &'a lexer::SyntaxElement,
    pub errors: Vec<crate::Error>,
}

impl<'a> crate::FromSyntax<'a> for InvalidNode<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Self {
        let errors = Vec::from([crate::Error::UnexpectedSyntax {
            syntax: syntax.clone(),
        }]);

        Self { syntax, errors }
    }
}
