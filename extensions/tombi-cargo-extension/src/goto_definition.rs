use crate::{get_dependencies_crate_path_location, get_workspace_cargo_toml_location};
use itertools::Itertools;
use tombi_config::TomlVersion;
use tombi_extension::DefinitionLocations;
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

    let locations = if match_accessors!(accessors[..1], ["workspace"]) {
        goto_definition_for_workspace_cargo_toml(
            document_tree,
            accessors,
            &cargo_toml_path,
            toml_version,
        )
    } else {
        goto_definition_for_crate_cargo_toml(
            document_tree,
            accessors,
            &cargo_toml_path,
            toml_version,
        )
    }?;

    if locations.is_empty() {
        return Ok(None);
    }

    Ok(Some(DefinitionLocations(locations)))
}

fn goto_definition_for_workspace_cargo_toml(
    workspace_document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    workspace_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    if match_accessors!(accessors, ["workspace", "dependencies", _, "path"]) {
        match get_dependencies_crate_path_location(
            workspace_document_tree,
            accessors,
            workspace_cargo_toml_path,
            toml_version,
        )? {
            Some(location) => Ok(vec![location]),
            None => Ok(Vec::with_capacity(0)),
        }
    } else if match_accessors!(accessors, ["workspace", "members", _]) {
        get_dependencies_workspace_members(
            workspace_document_tree,
            accessors,
            workspace_cargo_toml_path,
            toml_version,
        )
    } else {
        Ok(Vec::with_capacity(0))
    }
}

fn goto_definition_for_crate_cargo_toml(
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    let location = if matches!(accessors.last(), Some(tombi_schema_store::Accessor::Key(key)) if key == "workspace")
    {
        get_workspace_cargo_toml_location(accessors, cargo_toml_path, toml_version, true)
    } else if match_accessors!(accessors, ["dependencies", _, "path"])
        || match_accessors!(accessors, ["dev-dependencies", _, "path"])
        || match_accessors!(accessors, ["build-dependencies", _, "path"])
    {
        get_dependencies_crate_path_location(
            document_tree,
            accessors,
            cargo_toml_path,
            toml_version,
        )
    } else {
        Ok(None)
    }?;

    match location {
        Some(location) => Ok(vec![location]),
        None => Ok(Vec::with_capacity(0)),
    }
}

fn get_dependencies_workspace_members(
    workspace_document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    workspace_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DefinitionLocation>, tower_lsp::jsonrpc::Error> {
    assert!(match_accessors!(accessors, ["workspace", "members", _]));
    let Some((_, tombi_document_tree::Value::String(member))) =
        dig_accessors(workspace_document_tree, accessors)
    else {
        return Ok(Vec::with_capacity(0));
    };

    let Some(workspace_dir_path) = workspace_cargo_toml_path.parent() else {
        return Ok(Vec::with_capacity(0));
    };

    let exclude_patterns =
        match tombi_document_tree::dig_keys(workspace_document_tree, &["workspace", "exclude"]) {
            Some((_, tombi_document_tree::Value::Array(exclude))) => exclude
                .iter()
                .filter_map(|member| match member {
                    tombi_document_tree::Value::String(member_pattern) => Some(member_pattern),
                    _ => None,
                })
                .collect_vec(),
            _ => Vec::with_capacity(0),
        };

    let mut locations = Vec::new();
    for (_, cargo_toml_path) in
        crate::find_package_cargo_toml_paths(&[member], &exclude_patterns, workspace_dir_path)
    {
        let Ok(cargo_toml_uri) = tower_lsp::lsp_types::Url::from_file_path(&cargo_toml_path) else {
            continue;
        };

        let Some(member_document_tree) = crate::load_cargo_toml(&cargo_toml_path, toml_version)
        else {
            continue;
        };

        let Some((package_name_key, tombi_document_tree::Value::String(_package_name))) =
            tombi_document_tree::dig_keys(&member_document_tree, &["package", "name"])
        else {
            continue;
        };

        locations.push(tombi_extension::DefinitionLocation::new(
            cargo_toml_uri,
            package_name_key.range(),
        ));
    }

    Ok(locations)
}
