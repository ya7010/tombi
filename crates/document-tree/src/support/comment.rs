use ast::{support::string::parse_literal_string, AstToken};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    String(#[from] crate::support::string::ParseError),
}

pub fn try_from_comment(value: &str) -> Result<String, ParseError> {
    let comment = parse_literal_string(&value[1..], false)?;

    Ok(comment)
}

pub(crate) fn try_new_comment(node: &ast::Comment) -> Result<String, crate::Error> {
    try_from_comment(node.syntax().text()).map_err(|error| crate::Error::ParseCommentError {
        error,
        range: node.syntax().range(),
    })
}
