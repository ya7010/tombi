mod context;
mod error;
mod format;
mod options;

use std::borrow::Cow;

use ast::AstNode;
use context::Context;
pub use error::Error;
use format::Format;
pub use options::Options;

pub fn format(source: &str, options: &Options) -> Result<String, crate::Error> {
    let p = parser::parse(source);
    if let Some(root) = ast::Root::cast(p.into_syntax_node()) {
        Ok(root.format(&Context {
            options: Cow::Borrowed(options),
        }))
    } else {
        Err(crate::Error::ParseInvalid)
    }
}

#[inline]
fn children_kinds<T: From<u16>>(parent: &syntax::SyntaxNode) -> Vec<T> {
    parent
        .children()
        .into_iter()
        .map(|it| T::from(it.kind() as u16))
        .collect()
}
