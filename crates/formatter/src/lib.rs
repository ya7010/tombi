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

pub fn format(source: &str) -> Result<String, crate::Error> {
    format_with_option(source, &Options::default())
}

pub fn format_with_option(source: &str, options: &Options) -> Result<String, crate::Error> {
    let p = parser::parse(source);
    if p.errors().len() == 0 {
        let root = ast::Root::cast(p.into_syntax_node()).unwrap();
        Ok(root.format(&Context {
            options: Cow::Borrowed(options),
        }))
    } else {
        Err(crate::Error::from_syntax_error(source, &p.errors()))
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
