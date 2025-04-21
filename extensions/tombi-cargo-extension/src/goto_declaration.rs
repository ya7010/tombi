use tombi_ast::AstNode;
use tombi_config::TomlVersion;
use tombi_document_tree::TryIntoDocumentTree;
use tower_lsp::lsp_types::{Location, TextDocumentIdentifier, Url};

pub async fn goto_declaration(
    text_document: TextDocumentIdentifier,
    keys: &[tombi_document_tree::Key],
    toml_version: TomlVersion,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }

    if keys.last().map(|key| key.value()) != Some("workspace") {
        return Ok(None);
    }

    if let Some(workspace_location) =
        find_workspace_root(&text_document.uri, &keys[..keys.len() - 1], toml_version).await
    {
        return Ok(Some(workspace_location));
    }

    Ok(None)
}

/// Find the workspace root Cargo.toml for the given dependency
async fn find_workspace_root(
    cargo_toml_uri: &Url,
    keys: &[tombi_document_tree::Key],
    toml_version: TomlVersion,
) -> Option<Location> {
    // Get the directory of the current Cargo.toml
    let current_dir = cargo_toml_uri.to_file_path().ok()?;
    let mut current_dir: &std::path::Path = current_dir.as_ref();

    while let Some(target_dir) = current_dir.parent() {
        current_dir = target_dir;
        let workspace_cargo_toml = current_dir.join("Cargo.toml");

        if workspace_cargo_toml.exists() {
            tracing::error!(?workspace_cargo_toml);
            let Some(toml_text) = std::fs::read_to_string(&workspace_cargo_toml).ok() else {
                continue;
            };

            let Some(root) =
                tombi_ast::Root::cast(tombi_parser::parse(&toml_text).into_syntax_node())
            else {
                continue;
            };

            let Ok(document_tree) = root.try_into_document_tree(toml_version) else {
                continue;
            };

            let Some((mut target_key, mut value)) = document_tree.get_key_value("workspace") else {
                continue;
            };

            for key in keys {
                let tombi_document_tree::Value::Table(table) = value else {
                    return None;
                };

                let Some((next_key, next_value)) = table.get_key_value(key) else {
                    return None;
                };

                target_key = next_key;
                value = next_value;
            }

            let Ok(workspace_cargo_toml_uri) = Url::from_file_path(workspace_cargo_toml) else {
                return None;
            };

            return Some(Location::new(
                workspace_cargo_toml_uri,
                target_key.range().into(),
            ));
        }
    }

    None
}
