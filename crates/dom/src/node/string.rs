#[derive(Debug, Clone, Copy)]
pub enum StringKind {
    Basic,
    MultiLineBasic,
    Literal,
    MultiLineLiteral,
}

#[derive(Debug, Clone)]
pub struct StringNode<'a> {
    pub kind: StringKind,
    pub value: Option<&'a str>,
    pub syntax: &'a lexer::SyntaxElement,
    pub errors: Vec<crate::Error>,
}

impl<'a> crate::FromSyntax<'a> for StringNode<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Self {
        use lexer::Token::*;

        let mut errors = Vec::new();
        let kind = match syntax.kind() {
            BASIC_STRING => StringKind::Basic,
            MULTI_LINE_BASIC_STRING => StringKind::MultiLineBasic,
            LITERAL_STRING => StringKind::Literal,
            MULTI_LINE_LITERAL_STRING => StringKind::MultiLineLiteral,
            _ => unreachable!("invalid string kind: {syntax:#?}"),
        };
        let value = syntax.as_token().map(|t| t.text());

        if let Some(value) = value {
            Self {
                kind,
                value: Some(value),
                syntax,
                errors,
            }
        } else {
            errors.push(crate::Error::InvalidStringValue {
                syntax: syntax.clone(),
            });

            Self {
                kind,
                value: None,
                syntax,
                errors,
            }
        }
    }
}
