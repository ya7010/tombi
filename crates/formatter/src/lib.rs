mod error;
mod options;

use dom::TryFromSyntax;
pub use error::Error;
pub use options::Options;

pub fn format(source: &str, _options: &Options) -> Result<(), crate::Error> {
    let p = parser::parse(source);
    let syntax = p.syntax_node().into();
    let dom = dom::Node::try_from_syntax(&syntax).map_err(|e| crate::Error::Dom(e))?;

    println!("source: {:#?}", source);
    println!("dom: {:#?}", dom);

    Ok(())
}
