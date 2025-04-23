mod goto_declaration;
mod goto_definition;

pub use goto_declaration::goto_declaration;
pub use goto_definition::goto_definition;
use tombi_ast::AstNode;
use tombi_config::TomlVersion;
use tombi_document_tree::TryIntoDocumentTree;
use tower_lsp::lsp_types::{Location, Url};

fn load_pyproject_toml(
    pyproject_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<tombi_document_tree::DocumentTree> {
    let Some(toml_text) = std::fs::read_to_string(pyproject_toml_path).ok() else {
        return None;
    };

    let Some(root) = tombi_ast::Root::cast(tombi_parser::parse(&toml_text).into_syntax_node())
    else {
        return None;
    };

    root.try_into_document_tree(toml_version).ok()
}

fn find_workspace_pyproject_toml(
    pyproject_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<(std::path::PathBuf, tombi_document_tree::DocumentTree)> {
    let Some(mut current_dir) = pyproject_toml_path.parent() else {
        return None;
    };

    while let Some(target_dir) = current_dir.parent() {
        current_dir = target_dir;
        let workspace_pyproject_toml_path = current_dir.join("pyproject.toml");

        if workspace_pyproject_toml_path.exists() {
            let Some(document_tree) =
                load_pyproject_toml(&workspace_pyproject_toml_path, toml_version)
            else {
                continue;
            };

            // Check if this pyproject.toml has a [tool.uv.workspace] section
            if has_uv_workspace_section(&document_tree) {
                return Some((workspace_pyproject_toml_path, document_tree));
            };
        }
    }

    None
}

fn has_uv_workspace_section(document_tree: &tombi_document_tree::DocumentTree) -> bool {
    // Check if the document tree has [tool.uv.workspace] section
    let tool_key = create_key("tool", TomlVersion::V1_0_0);
    let uv_key = create_key("uv", TomlVersion::V1_0_0);
    let workspace_key = create_key("workspace", TomlVersion::V1_0_0);

    if let Some((_, value)) = document_tree.get_key_value(&tool_key) {
        if let tombi_document_tree::Value::Table(tool_table) = value {
            if let Some((_, uv_value)) = tool_table.get_key_value(&uv_key) {
                if let tombi_document_tree::Value::Table(uv_table) = uv_value {
                    return uv_table.contains_key(workspace_key.value());
                }
            }
        }
    }
    false
}

fn create_key(key_str: &str, toml_version: TomlVersion) -> tombi_document_tree::Key {
    tombi_document_tree::Key::try_new(
        tombi_document_tree::KeyKind::BareKey,
        key_str.to_string(),
        tombi_text::Range::default(),
        toml_version,
    )
    .unwrap()
}

fn get_package_pyproject_toml(
    workspace_pyproject_toml_path: &std::path::Path,
    package_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<(std::path::PathBuf, tombi_document_tree::DocumentTree)> {
    let mut package_path = package_path.to_path_buf();
    if !package_path.is_absolute() {
        if let Some(workspace_dir) = workspace_pyproject_toml_path.parent() {
            package_path = workspace_dir.join(package_path);
        }
    }

    let package_pyproject_toml_path = package_path.join("pyproject.toml");
    if !package_pyproject_toml_path.exists() {
        return None;
    }

    let Some(document_tree) = load_pyproject_toml(&package_pyproject_toml_path, toml_version)
    else {
        return None;
    };

    Some((package_pyproject_toml_path, document_tree))
}

/// Get the location of the workspace pyproject.toml.
///
/// ```toml
/// [project]
/// name = "example"
/// version = "0.1.0"
/// dependencies = ["other-package"]
///
/// [tool.uv.sources]
/// other-package = { workspace = true }
/// ```
fn get_workspace_pyproject_toml_location(
    keys: &[tombi_document_tree::Key],
    pyproject_toml_path: &std::path::Path,
    toml_version: TomlVersion,
    jump_to_package: bool,
) -> Result<Option<Location>, tower_lsp::jsonrpc::Error> {
    assert!(matches!(
        keys.last().map(|key| key.value()),
        Some("workspace")
    ));

    let Some((workspace_pyproject_toml_path, document_tree)) =
        find_workspace_pyproject_toml(pyproject_toml_path, toml_version)
    else {
        return Ok(None);
    };

    // Path to follow in workspace: tool -> uv -> workspace
    let tool_key = create_key("tool", toml_version);
    let uv_key = create_key("uv", toml_version);
    let workspace_key = create_key("workspace", toml_version);

    // Adjust keys based on the exact pyproject.toml structure we're looking for
    let mut sanitized_keys = vec![];

    // If we're coming from a sources section, we handle it specially
    if keys.len() >= 3
        && keys[0].value() == "tool"
        && keys[1].value() == "uv"
        && keys[2].value() == "sources"
    {
        // We're in the [tool.uv.sources] section
        sanitized_keys.push(create_key("tool", toml_version));
        sanitized_keys.push(create_key("uv", toml_version));
        sanitized_keys.push(create_key("sources", toml_version));

        if keys.len() > 3 {
            sanitized_keys.push(keys[3].clone());
        }
    } else {
        // For other paths, just use the original keys minus the workspace part
        sanitized_keys = keys[..keys.len() - 1].to_vec();
    }

    let mut workspace_keys = Vec::new();
    workspace_keys.push(tool_key.clone());
    workspace_keys.push(uv_key.clone());
    workspace_keys.push(workspace_key.clone());

    let Some((target_key, _)) = dig_keys(&document_tree, &workspace_keys) else {
        return Ok(None);
    };

    if jump_to_package {
        // If we have [tool.uv.sources] and trying to jump to a workspace package
        if keys.len() >= 4
            && keys[0].value() == "tool"
            && keys[1].value() == "uv"
            && keys[2].value() == "sources"
        {
            // Get the workspace members from [tool.uv.workspace]
            if let Some((_, workspace_value)) = dig_keys(&document_tree, &workspace_keys) {
                if let tombi_document_tree::Value::Table(workspace_table) = workspace_value {
                    if let Some((_, members_value)) =
                        workspace_table.get_key_value(&create_key("members", toml_version))
                    {
                        if let tombi_document_tree::Value::Array(members_array) = members_value {
                            // Try to find the package in the workspace members
                            for member in members_array.iter() {
                                if let tombi_document_tree::Value::String(member_glob) = member {
                                    // For each member glob pattern, try to find matching directories
                                    if let Some(workspace_dir) =
                                        workspace_pyproject_toml_path.parent()
                                    {
                                        // Simple case: if the glob is like "packages/*"
                                        if member_glob.value().ends_with('*') {
                                            let base_dir =
                                                member_glob.value().trim_end_matches('*');
                                            let packages_dir = workspace_dir.join(base_dir);

                                            if packages_dir.exists() && packages_dir.is_dir() {
                                                if let Ok(entries) = std::fs::read_dir(packages_dir)
                                                {
                                                    for entry in entries.flatten() {
                                                        if entry.path().is_dir() {
                                                            let package_name = entry.file_name();
                                                            if package_name.to_string_lossy()
                                                                == keys[3].value()
                                                            {
                                                                if let Some((
                                                                    package_toml_path,
                                                                    _,
                                                                )) = get_package_pyproject_toml(
                                                                    &workspace_pyproject_toml_path,
                                                                    &entry.path(),
                                                                    toml_version,
                                                                ) {
                                                                    return Ok(Some(Location::new(
                                                                        Url::from_file_path(package_toml_path).unwrap(),
                                                                        tombi_text::Range::default().into(),
                                                                    )));
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let Ok(workspace_pyproject_toml_uri) = Url::from_file_path(&workspace_pyproject_toml_path)
    else {
        return Ok(None);
    };

    Ok(Some(Location::new(
        workspace_pyproject_toml_uri,
        target_key.range().into(),
    )))
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
