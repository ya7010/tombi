mod goto_declaration;
mod goto_definition;

pub use goto_declaration::goto_declaration;
pub use goto_definition::goto_definition;
use itertools::Itertools;
use tombi_ast::AstNode;
use tombi_config::TomlVersion;
use tombi_document_tree::TryIntoDocumentTree;
use tower_lsp::lsp_types::{Location, Url};

fn load_cargo_toml(
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<tombi_document_tree::DocumentTree> {
    let Some(toml_text) = std::fs::read_to_string(cargo_toml_path).ok() else {
        return None;
    };

    let Some(root) = tombi_ast::Root::cast(tombi_parser::parse(&toml_text).into_syntax_node())
    else {
        return None;
    };

    root.try_into_document_tree(toml_version).ok()
}

fn find_workspace_cargo_toml(
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<(std::path::PathBuf, tombi_document_tree::DocumentTree)> {
    let Some(mut current_dir) = cargo_toml_path.parent() else {
        return None;
    };

    while let Some(target_dir) = current_dir.parent() {
        current_dir = target_dir;
        let workspace_cargo_toml_path = current_dir.join("Cargo.toml");

        if workspace_cargo_toml_path.exists() {
            let Some(document_tree) = load_cargo_toml(&workspace_cargo_toml_path, toml_version)
            else {
                continue;
            };

            if document_tree.contains_key("workspace") {
                return Some((workspace_cargo_toml_path, document_tree));
            };
        }
    }

    None
}

fn get_subcrate_cargo_toml(
    workspace_cargo_toml_path: &std::path::Path,
    subcrate_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<(std::path::PathBuf, tombi_document_tree::DocumentTree)> {
    let mut subcrate_path = subcrate_path.to_path_buf();
    if !subcrate_path.is_absolute() {
        if let Some(workspace_dir) = workspace_cargo_toml_path.parent() {
            subcrate_path = workspace_dir.join(subcrate_path);
        }
    }

    let subcrate_cargo_toml_path = subcrate_path.join("Cargo.toml");
    if !subcrate_cargo_toml_path.exists() {
        return None;
    }

    let Some(document_tree) = load_cargo_toml(&subcrate_cargo_toml_path, toml_version) else {
        return None;
    };

    Some((subcrate_cargo_toml_path, document_tree))
}

/// Get the location of the workspace Cargo.toml.
///
/// ```toml
/// [project]
/// name = "my_project"
/// version = { workspace = true }
///
/// [dependencies]
/// tombi-ast = { workspace = true }
/// ```
fn get_workspace_cargo_toml_location(
    keys: &[tombi_document_tree::Key],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
    jump_to_subcrate: bool,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    assert!(matches!(
        keys.last().map(|key| key.value()),
        Some("workspace")
    ));

    let Some((workspace_cargo_toml_path, document_tree)) =
        find_workspace_cargo_toml(cargo_toml_path, toml_version)
    else {
        return Ok(None);
    };

    let workspace_key = create_key("workspace", toml_version);

    let keys = {
        let mut sanitized_keys = vec![sanitize_dependency_key(keys[0].to_owned(), toml_version)];
        sanitized_keys.extend(
            keys[1..]
                .iter()
                .map(|key| sanitize_dependency_key(key.to_owned(), toml_version)),
        );
        sanitized_keys
    };

    let Some((target_key, value)) = dig_keys(
        &document_tree,
        &std::iter::once(workspace_key)
            .chain(keys[..keys.len() - 1].iter().cloned())
            .collect_vec(),
    ) else {
        return Ok(None);
    };

    if jump_to_subcrate {
        if matches!(
            keys.first().map(|key| key.value()),
            Some("dependencies" | "dev-dependencies" | "build-dependencies")
        ) {
            if matches!(value, tombi_document_tree::Value::Table(table)) {
                let table = match value {
                    tombi_document_tree::Value::Table(t) => t,
                    _ => unreachable!(),
                };

                if let Some(path_value) = table.get("path") {
                    if matches!(path_value, tombi_document_tree::Value::String(_)) {
                        let subcrate_path = match path_value {
                            tombi_document_tree::Value::String(path) => path,
                            _ => unreachable!(),
                        };

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
            }
        }
    }

    let Ok(workspace_cargo_toml_uri) = Url::from_file_path(&workspace_cargo_toml_path) else {
        return Ok(None);
    };

    Ok(Some(Location::new(
        workspace_cargo_toml_uri,
        target_key.range().into(),
    )))
}

/// Get the location of the crate path in the workspace.
///
/// ```toml
/// [workspace.dependencies]
/// tombi-ast = { path = "crates/tombi-ast" }
/// ```
fn get_dependencies_crate_path_location(
    keys: &[tombi_document_tree::Key],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    assert!(matches!(
        keys.iter().map(|key| key.value()).collect_vec().as_slice(),
        ["workspace", "dependencies", _, "path"]
            | [
                "dependencies" | "dev-dependencies" | "build-dependencies",
                _,
                "path"
            ]
    ));

    let Some(document_tree) = load_cargo_toml(cargo_toml_path, toml_version) else {
        return Ok(None);
    };

    let Some((_, value)) = dig_keys(&document_tree, &keys) else {
        return Ok(None);
    };

    if matches!(value, tombi_document_tree::Value::String(subcrate_path)) {
        let subcrate_path = match value {
            tombi_document_tree::Value::String(path) => path,
            _ => unreachable!(),
        };

        if let Some((subcrate_cargo_toml_path, _)) = get_subcrate_cargo_toml(
            &cargo_toml_path,
            std::path::Path::new(subcrate_path.value()),
            toml_version,
        ) {
            return Ok(Some(Location::new(
                Url::from_file_path(subcrate_cargo_toml_path).unwrap(),
                tombi_text::Range::default().into(),
            )));
        }
    }

    Ok(None)
}

fn dig_keys<'a>(
    document_tree: &'a tombi_document_tree::DocumentTree,
    keys: &[tombi_document_tree::Key],
) -> Option<(&'a tombi_document_tree::Key, &'a tombi_document_tree::Value)> {
    if keys.is_empty() {
        return None;
    }
    let Some((mut key, mut value)) = document_tree.get_key_value(&keys[0]) else {
        return None;
    };
    for k in keys[1..].iter() {
        let tombi_document_tree::Value::Table(table) = value else {
            return None;
        };

        let Some((next_key, next_value)) = table.get_key_value(k) else {
            return None;
        };

        key = next_key;
        value = next_value;
    }

    Some((key, value))
}

/// Create a new bare key with the given name.
fn create_key(key_str: &str, toml_version: TomlVersion) -> tombi_document_tree::Key {
    tombi_document_tree::Key::try_new(
        tombi_document_tree::KeyKind::BareKey,
        key_str.to_string(),
        tombi_text::Range::default(),
        toml_version,
    )
    .unwrap()
}

/// Sanitize the dependency key to be "dependencies" if it is "dev-dependencies" or "build-dependencies".
///
/// This is because the dependency key is always "dependencies" in the workspace Cargo.toml.
fn sanitize_dependency_key(
    key: tombi_document_tree::Key,
    toml_version: TomlVersion,
) -> tombi_document_tree::Key {
    if matches!(key.value(), "dev-dependencies" | "build-dependencies") {
        create_key("dependencies", toml_version)
    } else {
        key
    }
}
