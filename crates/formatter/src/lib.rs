mod error;
mod format;
mod formatter;
mod options;

use std::borrow::Cow;

use ast::AstNode;
use diagnostics::Diagnostic;
pub use error::Error;
use format::Format;
pub use formatter::Formatter;
pub use options::Options;

pub fn format(source: &str) -> Result<String, Vec<Diagnostic>> {
    format_with_option(source, &Options::default())
}

pub fn format_with_option(source: &str, options: &Options) -> Result<String, Vec<Diagnostic>> {
    let p = parser::parse(source);
    let errors = p.errors();

    let root = ast::Root::cast(p.into_syntax_node()).unwrap();
    tracing::trace!("ast: {:#?}", root);

    if errors.len() == 0 {
        let mut formatted_text = String::new();
        root.fmt(&mut Formatter::new_with_options(
            &mut formatted_text,
            options.clone(),
        ))
        .unwrap();

        Ok(formatted_text)
    } else {
        Err(errors
            .into_iter()
            .map(|error| Diagnostic::new_error(error.message(), source, error.range()))
            .collect())
    }
}
