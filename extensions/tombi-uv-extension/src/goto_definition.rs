use tombi_config::TomlVersion;
use tower_lsp::lsp_types::{Location, TextDocumentIdentifier};

use crate::get_workspace_pyproject_toml_location;

pub async fn goto_definition(
    text_document: TextDocumentIdentifier,
    keys: &[tombi_document_tree::Key],
    toml_version: TomlVersion,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    // Check if current file is pyproject.toml
    if !text_document.uri.path().ends_with("pyproject.toml") {
        return Ok(None);
    }
    let Ok(pyproject_toml_path) = text_document.uri.to_file_path() else {
        return Ok(None);
    };

    if keys.len() >= 3 && keys[0].value() == "tool" && keys[1].value() == "uv" {
        goto_definition_for_uv_pyproject_toml(&keys, &pyproject_toml_path, toml_version)
    } else {
        Ok(None)
    }
}

fn goto_definition_for_uv_pyproject_toml(
    keys: &[tombi_document_tree::Key],
    pyproject_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    // Handle navigation to workspace packages when {workspace = true} is specified
    if keys.len() >= 4
        && keys[0].value() == "tool"
        && keys[1].value() == "uv"
        && keys[2].value() == "sources"
        && keys.last().map(|key| key.value()) == Some("workspace")
    {
        get_workspace_pyproject_toml_location(&keys, pyproject_toml_path, toml_version, true)
    } else {
        Ok(None)
    }
}
