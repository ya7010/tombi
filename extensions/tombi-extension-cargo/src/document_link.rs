use crate::{
    find_workspace_cargo_toml, get_path_crate_cargo_toml, goto_workspace_member_crates,
    load_cargo_toml,
};
use tombi_config::TomlVersion;
use tombi_document_tree::dig_keys;
use tower_lsp::lsp_types::{TextDocumentIdentifier, Url};

pub enum DocumentLinkToolTip {
    GitRepository,
    CrateIo,
    CargoToml,
    CargoTomlFirstMember,
}

impl std::fmt::Display for DocumentLinkToolTip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentLinkToolTip::GitRepository => write!(f, "Open Git Repository"),
            DocumentLinkToolTip::CrateIo => write!(f, "Open crate.io"),
            DocumentLinkToolTip::CargoToml => write!(f, "Open Cargo.toml"),
            DocumentLinkToolTip::CargoTomlFirstMember => {
                write!(f, "Open first Cargo.toml in members")
            }
        }
    }
}

pub async fn document_link(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    toml_version: TomlVersion,
) -> Result<Option<Vec<tombi_extension::DocumentLink>>, tower_lsp::jsonrpc::Error> {
    // Check if current file is Cargo.toml
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }
    let Some(cargo_toml_path) = text_document.uri.to_file_path().ok() else {
        return Ok(None);
    };

    let mut document_links = vec![];

    if document_tree.contains_key("workspace") {
        document_links.extend(document_link_for_workspace_cargo_toml(
            document_tree,
            &cargo_toml_path,
            toml_version,
        )?);
    } else {
        document_links.extend(document_link_for_crate_cargo_toml(
            document_tree,
            &cargo_toml_path,
            toml_version,
        )?);
    }

    if document_links.is_empty() {
        return Ok(None);
    }

    Ok(Some(document_links))
}

fn document_link_for_workspace_cargo_toml(
    workspace_document_tree: &tombi_document_tree::DocumentTree,
    workspace_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DocumentLink>, tower_lsp::jsonrpc::Error> {
    let mut total_document_links = vec![];

    if let Some((_, tombi_document_tree::Value::Table(dependencies))) =
        dig_keys(workspace_document_tree, &["workspace", "dependencies"])
    {
        total_document_links.extend(document_link_for_workspace_depencencies(
            dependencies,
            workspace_cargo_toml_path,
            toml_version,
        )?);
    }
    if let Some((_, tombi_document_tree::Value::Array(members))) =
        dig_keys(workspace_document_tree, &["workspace", "members"])
    {
        for (i, member) in members.values().iter().enumerate() {
            let member = match member {
                tombi_document_tree::Value::String(member) => member,
                _ => continue,
            };
            let mut member_document_links = goto_workspace_member_crates(
                workspace_document_tree,
                &[
                    tombi_schema_store::Accessor::Key("workspace".to_string()),
                    tombi_schema_store::Accessor::Key("members".to_string()),
                    tombi_schema_store::Accessor::Index(i),
                ],
                workspace_cargo_toml_path,
                toml_version,
            )?
            .into_iter()
            .filter_map(|location| {
                Url::from_file_path(location.cargo_toml_path)
                    .map(|target| tombi_extension::DocumentLink {
                        target,
                        range: member.unquoted_range(),
                        tooltip: DocumentLinkToolTip::CargoTomlFirstMember.to_string(),
                    })
                    .ok()
            });
            match member_document_links.size_hint() {
                (_, Some(n)) if n > 0 => {
                    if let Some(mut document_link) = member_document_links.next() {
                        if n == 1 {
                            document_link.tooltip = DocumentLinkToolTip::CargoToml.to_string();
                        }
                        total_document_links.push(document_link);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(total_document_links)
}

fn document_link_for_workspace_depencencies(
    dependencies: &tombi_document_tree::Table,
    workspace_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DocumentLink>, tower_lsp::jsonrpc::Error> {
    let mut total_document_links = vec![];
    for (crate_name, crate_value) in dependencies.key_values() {
        if let Ok(document_links) = document_link_for_dependency(
            crate_name,
            crate_value,
            workspace_cargo_toml_path,
            workspace_cargo_toml_path,
            toml_version,
        ) {
            total_document_links.extend(document_links);
        }
    }

    Ok(total_document_links)
}

fn document_link_for_crate_cargo_toml(
    crate_document_tree: &tombi_document_tree::DocumentTree,
    crate_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DocumentLink>, tower_lsp::jsonrpc::Error> {
    let mut total_dependencies = vec![];
    for key in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some((_, tombi_document_tree::Value::Table(dependencies))) =
            dig_keys(crate_document_tree, &[key])
        {
            total_dependencies.extend(dependencies.key_values());
        }
    }

    let mut total_document_links = vec![];
    if let Some((workspace_cargo_toml_path, workspace_document_tree)) =
        find_workspace_cargo_toml(crate_cargo_toml_path, toml_version)
    {
        for (crate_key, crate_value) in total_dependencies {
            if let tombi_document_tree::Value::Table(crate_table) = crate_value {
                if let Some((workspace_key, tombi_document_tree::Value::Boolean(is_workspace))) =
                    crate_table.get_key_value("workspace")
                {
                    if is_workspace.value() {
                        let Some((_, tombi_document_tree::Value::Table(dependencies))) =
                            dig_keys(&workspace_document_tree, &["workspace", "dependencies"])
                        else {
                            continue;
                        };
                        if let Some(workspace_crate_value) = dependencies.get(&crate_key) {
                            if let Ok(document_links) = document_link_for_dependency(
                                crate_key,
                                workspace_crate_value,
                                &workspace_cargo_toml_path,
                                &workspace_cargo_toml_path,
                                toml_version,
                            ) {
                                for document_link in document_links {
                                    total_document_links.push(document_link.clone());
                                    let mut document_link = document_link;
                                    document_link.range =
                                        workspace_key.range() + is_workspace.range();
                                    total_document_links.push(document_link)
                                }
                            }
                        }
                        continue;
                    }
                }
            }
            if let Ok(document_links) = document_link_for_dependency(
                crate_key,
                crate_value,
                crate_cargo_toml_path,
                &workspace_cargo_toml_path,
                toml_version,
            ) {
                total_document_links.extend(document_links);
            }
        }
    } else {
        for (crate_name, crate_value) in total_dependencies {
            if let Ok(document_links) = document_link_for_dependency(
                crate_name,
                crate_value,
                crate_cargo_toml_path,
                crate_cargo_toml_path,
                toml_version,
            ) {
                total_document_links.extend(document_links);
            }
        }
    }

    Ok(total_document_links)
}

fn document_link_for_dependency(
    crate_key: &tombi_document_tree::Key,
    crate_value: &tombi_document_tree::Value,
    crate_cargo_toml_path: &std::path::Path,
    workspace_cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Result<Vec<tombi_extension::DocumentLink>, tower_lsp::jsonrpc::Error> {
    let mut registory = "https://crates.io/crates".to_string();

    if let tombi_document_tree::Value::Table(table) = crate_value {
        if let Some(tombi_document_tree::Value::String(subcrate_path)) = table.get("path") {
            if let Some((subcrate_cargo_toml_path, subcrate_document_tree)) =
                get_path_crate_cargo_toml(
                    crate_cargo_toml_path,
                    std::path::Path::new(subcrate_path.value()),
                    toml_version,
                )
            {
                if let Some((_, tombi_document_tree::Value::String(package_name))) =
                    tombi_document_tree::dig_keys(&subcrate_document_tree, &["package", "name"])
                {
                    let package_name_check =
                        if let Some(tombi_document_tree::Value::String(package_name)) =
                            table.get("package")
                        {
                            package_name.value() == crate_key.value()
                        } else {
                            package_name.value() == crate_key.value()
                        };
                    if package_name_check {
                        let Ok(target) = Url::from_file_path(subcrate_cargo_toml_path) else {
                            return Ok(Vec::with_capacity(0));
                        };
                        return Ok(vec![
                            tombi_extension::DocumentLink {
                                target: target.clone(),
                                range: crate_key.unquoted_range(),
                                tooltip: DocumentLinkToolTip::CargoToml.to_string(),
                            },
                            tombi_extension::DocumentLink {
                                target,
                                range: subcrate_path.unquoted_range(),
                                tooltip: DocumentLinkToolTip::CargoToml.to_string(),
                            },
                        ]);
                    }
                }
            }
        }
        if table.contains_key("workspace") {
            // At this stage, the workspace Cargo.toml has already been moved, so this condition is ignored.
            return Ok(Vec::with_capacity(0));
        } else if let Some(tombi_document_tree::Value::String(git_url)) = table.get("git") {
            let target = if let Ok(target) = Url::parse(git_url.value()) {
                target
            } else if let Ok(target) = Url::from_file_path(git_url.value()) {
                target
            } else {
                return Ok(Vec::with_capacity(0));
            };

            return Ok(vec![tombi_extension::DocumentLink {
                range: crate_key.unquoted_range(),
                target,
                tooltip: DocumentLinkToolTip::GitRepository.to_string(),
            }]);
        }
        if let Some(tombi_document_tree::Value::String(registory_name)) = table.get("registory") {
            if let Some(workspace_directory) = workspace_cargo_toml_path.parent() {
                if let Some(cargo_toml_document_tree) = load_cargo_toml(
                    &workspace_directory.join(".cargo/config.toml"),
                    toml_version,
                ) {
                    if let Some(tombi_document_tree::Value::Table(registries)) =
                        cargo_toml_document_tree.get("registries")
                    {
                        if registries.contains_key(registory_name.value()) {
                            if let Some(tombi_document_tree::Value::Table(registory_table)) =
                                registries.get(registory_name.value())
                            {
                                if let Some(tombi_document_tree::Value::String(url)) =
                                    registory_table.get("index")
                                {
                                    registory = url.value().to_string();
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(tombi_document_tree::Value::String(package_name)) = table.get("package") {
            let Ok(target) = Url::parse(&format!("{registory}/{}", package_name.value())) else {
                return Ok(Vec::with_capacity(0));
            };
            return Ok(vec![tombi_extension::DocumentLink {
                range: crate_key.unquoted_range(),
                target,
                tooltip: DocumentLinkToolTip::CrateIo.to_string(),
            }]);
        }
    }

    let Ok(target) = Url::parse(&format!("{registory}/{}", crate_key.value())) else {
        return Ok(Vec::with_capacity(0));
    };

    Ok(vec![tombi_extension::DocumentLink {
        range: crate_key.unquoted_range(),
        target,
        tooltip: DocumentLinkToolTip::CrateIo.to_string(),
    }])
}
