use crate::{get_dependencies_crate_path_location, get_workspace_cargo_toml_location};
use itertools::Itertools;
use tombi_config::TomlVersion;
use tower_lsp::lsp_types::TextDocumentIdentifier;

pub async fn goto_definition(
    text_document: &TextDocumentIdentifier,
    keys: &[tombi_document_tree::Key],
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocations>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }
    let Ok(cargo_toml_path) = text_document.uri.to_file_path() else {
        return Ok(None);
    };

    let keys = keys.iter().map(|key| key.value()).collect_vec();
    let keys = keys.as_slice();

    let definitions = if keys.first() == Some(&"workspace") {
        goto_definition_for_workspace_cargo_toml(&keys, &cargo_toml_path, toml_version)
    } else {
        goto_definition_for_crate_cargo_toml(&keys, &cargo_toml_path, toml_version)
    }?
    .map(Into::into);

    Ok(definitions)
}

fn goto_definition_for_workspace_cargo_toml(
    keys: &[&str],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    match keys {
        ["workspace", "dependencies", _, "path"] => {
            get_dependencies_crate_path_location(keys, cargo_toml_path, toml_version)
        }
        _ => Ok(None),
    }
}

fn goto_definition_for_crate_cargo_toml(
    keys: &[&str],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if keys.last() == Some(&"workspace") {
        get_workspace_cargo_toml_location(&keys, cargo_toml_path, toml_version, true)
    } else if matches!(
        keys,
        [
            "dependencies" | "dev-dependencies" | "build-dependencies",
            _,
            "path"
        ]
    ) {
        get_dependencies_crate_path_location(keys, cargo_toml_path, toml_version)
    } else {
        Ok(None)
    }
}
