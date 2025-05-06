use tombi_config::TomlVersion;
use tombi_schema_store::match_accessors;
use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::get_tool_uv_sources_workspace_location;

pub async fn goto_definition(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocations>, tower_lsp::jsonrpc::Error> {
    // Check if current file is pyproject.toml
    if !text_document.uri.path().ends_with("pyproject.toml") {
        return Ok(None);
    }
    let Ok(pyproject_toml_path) = text_document.uri.to_file_path() else {
        return Ok(None);
    };

    if match_accessors!(accessors, ["tool", "uv", "sources", _, "workspace"]) {
        Ok(get_tool_uv_sources_workspace_location(
            document_tree,
            accessors,
            &pyproject_toml_path,
            toml_version,
            true,
        )?
        .map(Into::into))
    } else {
        Ok(None)
    }
}
