mod error;

use ast::AstNode;
use document::IntoDocument;
use document_tree::IntoDocumentTreeAndErrors;
pub use error::Error;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use toml_version::TomlVersion;

/// Deserialize a TOML string into a Rust data structure.
///
/// # Note
///
/// This function is not yet implemented and will return an error.
/// The example below shows the expected usage once implemented.
///
/// # Examples
///
/// ```no_run
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     ip: String,
///     port: u16,
///     keys: Vec<String>,
/// }
///
/// let toml = r#"
/// ip = "127.0.0.1"
/// port = 8080
/// keys = ["key1", "key2"]
/// "#;
///
/// let config: Config = serde_tombi::from_str(toml).unwrap();
/// ```
pub fn from_str<T>(s: &str) -> Result<T, crate::de::Error>
where
    T: DeserializeOwned,
{
    let document = parse_str(s)?;
    from_document(document)
}

pub fn from_document<T>(document: document::Document) -> Result<T, crate::de::Error>
where
    T: DeserializeOwned,
{
    Ok(T::deserialize(&document)?)
}

/// Parse a TOML string into a Document.
pub fn parse_str(s: &str) -> Result<document::Document, crate::de::Error> {
    // Parse the source string using the parser
    let parsed = parser::parse(s);

    let errors = parsed.errors(TomlVersion::default()).collect_vec();
    // Check if there are any parsing errors
    if !errors.is_empty() {
        return Err(crate::de::Error::Parser(
            parsed.into_errors(TomlVersion::default()).collect_vec(),
        ));
    }

    // Cast the parsed result to an AST Root node
    let root = ast::Root::cast(parsed.into_syntax_node()).expect("AST Root must be present");

    // Convert the AST to a document tree
    let (document_tree, errors) = root
        .into_document_tree_and_errors(TomlVersion::default())
        .into();

    // Check for errors during document tree construction
    if !errors.is_empty() {
        return Err(crate::de::Error::DocumentTree(errors));
    }

    // Convert to a Document
    Ok(document_tree.into_document(TomlVersion::default()))
}
