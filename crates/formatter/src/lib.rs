mod error;
mod format;
mod options;

use ast::AstNode;
pub use error::Error;
use format::Format;
pub use options::Options;

pub fn format(source: &str, _options: &Options) -> Result<String, crate::Error> {
    let p = parser::parse(source);
    if let Some(root) = ast::Root::cast(p.syntax_node()) {
        Ok(root.format())
    } else {
        Err(crate::Error::ParseInvalid)
    }
}
