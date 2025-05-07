use tombi_config::TomlVersion;
use tombi_schema_store::match_accessors;
use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::goto_workspace_member;

pub async fn goto_declaration(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    // Check if current file is pyproject.toml
    if !text_document.uri.path().ends_with("pyproject.toml") {
        return Ok(None);
    }
    let Ok(pyproject_toml_path) = text_document.uri.to_file_path() else {
        return Ok(None);
    };

    if match_accessors!(accessors, ["tool", "uv", "sources", _, "workspace"]) {
        goto_workspace_member(
            document_tree,
            &accessors,
            &pyproject_toml_path,
            toml_version,
            false,
        )
    } else {
        Ok(None)
    }
}
