use tombi_config::TomlVersion;
use tower_lsp::lsp_types::{Location, TextDocumentIdentifier, Url};

use crate::{find_workspace_cargo_toml, get_subcrate_cargo_toml};

pub async fn goto_definition(
    text_document: TextDocumentIdentifier,
    keys: &[tombi_document_tree::Key],
    toml_version: TomlVersion,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }
    let Some(cargo_toml_path) = text_document.uri.to_file_path().ok() else {
        return Ok(None);
    };

    if keys.last().map(|key| key.value()) != Some("workspace") {
        return Ok(None);
    }

    let Some((workspace_cargo_toml_path, document_tree)) =
        find_workspace_cargo_toml(&cargo_toml_path, toml_version)
    else {
        return Ok(None);
    };

    let Some((mut target_key, mut value)) = document_tree.get_key_value("workspace") else {
        return Ok(None);
    };

    for key in keys[..keys.len() - 1].iter() {
        let tombi_document_tree::Value::Table(table) = value else {
            return Ok(None);
        };

        let Some((next_key, next_value)) = table.get_key_value(key) else {
            return Ok(None);
        };

        target_key = next_key;
        value = next_value;
    }

    if let tombi_document_tree::Value::Table(table) = value {
        // Support for subcrate
        //
        // ```toml
        // [workspace.dependencies]
        // tombi-ast = { path = "crates/tombi-ast" }
        // ```
        if let Some(tombi_document_tree::Value::String(subcrate_path)) = table.get("path") {
            if let Some((subcrate_cargo_toml_path, _)) = get_subcrate_cargo_toml(
                &workspace_cargo_toml_path,
                std::path::Path::new(subcrate_path.value()),
                toml_version,
            ) {
                return Ok(Some(Location::new(
                    Url::from_file_path(subcrate_cargo_toml_path).unwrap(),
                    tombi_text::Range::default().into(),
                )));
            }
        }
    }

    let Ok(workspace_cargo_toml_uri) = Url::from_file_path(workspace_cargo_toml_path) else {
        return Ok(None);
    };

    Ok(Some(Location::new(
        workspace_cargo_toml_uri,
        target_key.range().into(),
    )))
}
