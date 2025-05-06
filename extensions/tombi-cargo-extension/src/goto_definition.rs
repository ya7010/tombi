use crate::{get_dependencies_crate_path_location, get_workspace_cargo_toml_location};
use tombi_config::TomlVersion;
use tombi_schema_store::{match_accessors, Accessor};
use tower_lsp::lsp_types::TextDocumentIdentifier;

pub async fn goto_definition(
    text_document: &TextDocumentIdentifier,
    accessors: &[Accessor],
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
        if matches!(accessors.first(), Some(Accessor::Key(key)) if key == "workspace") {
            goto_definition_for_workspace_cargo_toml(accessors, &cargo_toml_path, toml_version)
        } else {
            goto_definition_for_crate_cargo_toml(accessors, &cargo_toml_path, toml_version)
        }?
        .map(Into::into);

    Ok(definitions)
}

fn goto_definition_for_workspace_cargo_toml(
    accessors: &[Accessor],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if match_accessors!(accessors, ["workspace", "dependencies", _, "path"]) {
        get_dependencies_crate_path_location(accessors, cargo_toml_path, toml_version)
    } else {
        Ok(None)
    }
}

fn goto_definition_for_crate_cargo_toml(
    accessors: &[Accessor],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if matches!(accessors.last(), Some(Accessor::Key(key)) if key == "workspace") {
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
