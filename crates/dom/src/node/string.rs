#[derive(Debug, Clone)]
pub struct StringNode<'a> {
    pub value: Option<&'a str>,
    pub syntax: &'a lexer::SyntaxElement,
    pub errors: Vec<crate::Error>,
}

impl<'a> crate::FromSyntax<'a> for StringNode<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Self {
        use lexer::Token::*;

        let mut errors = Vec::new();
        match syntax.kind() {
            BASIC_STRING => {
                let value = syntax.as_token().map(|t| t.text());
                if value.is_none() {
                    errors.push(crate::Error::InvalidStringValue {
                        syntax: syntax.clone(),
                    });
                }

                Self {
                    value,
                    syntax,
                    errors,
                }
            }
            _ => {
                errors.push(crate::Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });

                Self {
                    value: None,
                    syntax,
                    errors,
                }
            }
        }
    }
}
