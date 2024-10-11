mod error;
mod options;

use dom::TryFromSyntax;
pub use error::Error;
pub use options::Options;

pub fn format(source: &str, _options: &Options) -> Result<(), crate::Error> {
    let p = parser::parse(source);
    let syntax = p.into_syntax().into();
    let dom = dom::Node::try_from_syntax(&syntax).map_err(|e| crate::Error::Dom(e))?;

    println!("{:#?}", dom);

    Ok(())
}
