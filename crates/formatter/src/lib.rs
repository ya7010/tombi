mod definitions;
mod error;
mod format;
mod formatter;
mod options;

use ast::AstNode;
pub use definitions::Definitions;
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

    if errors.is_empty() {
        let mut formatted_text = String::new();
        let line_ending = {
            let mut f = Formatter::new_with_options(&mut formatted_text, options);
            root.fmt(&mut f).unwrap();
            f.line_ending()
        };

        Ok(formatted_text + line_ending)
    } else {
        Err(errors
            .into_iter()
            .map(|error| Diagnostic::new_error(error.message(), ((0, 0), (error.pos(), 0)).into()))
            .collect())
    }
}
