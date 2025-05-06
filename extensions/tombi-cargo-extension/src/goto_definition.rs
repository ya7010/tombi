use crate::{get_dependencies_crate_path_location, get_workspace_cargo_toml_location};
use tombi_config::TomlVersion;
use tombi_schema_store::{dig_accessors, match_accessors};
use tower_lsp::lsp_types::TextDocumentIdentifier;

pub async fn goto_definition(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocations>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }
    let Ok(cargo_toml_path) = text_document.uri.to_file_path() else {
        return Ok(None);
    };

    let definitions =
        if matches!(accessors.first(), Some(tombi_schema_store::Accessor::Key(key)) if key == "workspace") {
            goto_definition_for_workspace_cargo_toml(document_tree, accessors, &cargo_toml_path, toml_version)
        } else {
            goto_definition_for_crate_cargo_toml(document_tree, accessors, &cargo_toml_path, toml_version)
        }?
        .map(Into::into);

    Ok(definitions)
}

fn goto_definition_for_workspace_cargo_toml(
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if match_accessors!(accessors, ["workspace", "dependencies", _, "path"]) {
        get_dependencies_crate_path_location(accessors, cargo_toml_path, toml_version)
    } else if match_accessors!(accessors, ["workspace", "members", _]) {
        get_dependencies_workspace_members(document_tree, accessors, cargo_toml_path, toml_version)
    } else {
        Ok(None)
    }
}

fn goto_definition_for_crate_cargo_toml(
    _document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if matches!(accessors.last(), Some(tombi_schema_store::Accessor::Key(key)) if key == "workspace")
    {
        get_workspace_cargo_toml_location(accessors, cargo_toml_path, toml_version, true)
    } else if match_accessors!(accessors, ["dependencies", _, "path"])
        || match_accessors!(accessors, ["dev-dependencies", _, "path"])
        || match_accessors!(accessors, ["build-dependencies", _, "path"])
    {
        get_dependencies_crate_path_location(accessors, cargo_toml_path, toml_version)
    } else {
        Ok(None)
    }
}

fn get_dependencies_workspace_members(
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    _cargo_toml_path: &std::path::Path,
    _toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    let Some((_, tombi_document_tree::Value::String(_member))) =
        dig_accessors(document_tree, accessors)
    else {
        return Ok(None);
    };

    Ok(None)
}
