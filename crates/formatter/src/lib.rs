mod context;
mod error;
mod format;
mod options;

use std::borrow::Cow;

use ast::AstNode;
use context::Context;
use diagnostics::Diagnostic;
pub use error::Error;
use format::Format;
pub use options::Options;

pub fn format(source: &str) -> Result<String, Vec<Diagnostic>> {
    format_with_option(source, &Options::default())
}

pub fn format_with_option(source: &str, options: &Options) -> Result<String, Vec<Diagnostic>> {
    let p = parser::parse(source);
    if p.errors().len() == 0 {
        let root = ast::Root::cast(p.into_syntax_node()).unwrap();
        tracing::trace!("ast: {:#?}", root);
        Ok(root.format(&Context {
            options: Cow::Borrowed(options),
        }))
    } else {
        Err(p
            .errors()
            .into_iter()
            .map(|error| Diagnostic::new_error(error.message(), source, error.range()))
            .collect())
    }
}
