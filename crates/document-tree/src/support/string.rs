pub use ast::support::string::{
    from_bare_key, try_from_basic_string, try_from_literal_string,
    try_from_multi_line_basic_string, try_from_multi_line_literal_string, ParseError,
};
use ast::{support::string::parse_literal_string, AstToken};

pub fn try_new_comment(node: &ast::Comment) -> Result<String, crate::Error> {
    parse_literal_string(&node.syntax().text()[1..], false).map_err(|error| {
        crate::Error::ParseCommentError {
            error,
            range: node.syntax().range(),
        }
    })
}
