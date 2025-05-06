use tombi_config::TomlVersion;
use tombi_schema_store::match_accessors;
use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::get_workspace_cargo_toml_location;

pub async fn goto_declaration(
    text_document: &TextDocumentIdentifier,
    _document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }
    let Some(cargo_toml_path) = text_document.uri.to_file_path().ok() else {
        return Ok(None);
    };

    if match_accessors!(accessors[accessors.len() - 1..], ["workspace"]) {
        get_workspace_cargo_toml_location(accessors, &cargo_toml_path, toml_version, false)
    } else {
        Ok(None)
    }
}
