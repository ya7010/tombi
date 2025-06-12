use itertools::Itertools;
use serde::Deserialize;
use tombi_config::TomlVersion;
use tombi_extension::CompletionContent;
use tombi_extension::CompletionKind;
use tombi_schema_store::dig_accessors;
use tombi_schema_store::match_accessors;
use tombi_schema_store::Accessor;
use tombi_schema_store::HttpClient;
use tombi_version_sort::version_sort;
use tower_lsp::lsp_types::TextDocumentIdentifier;

#[derive(Debug, Deserialize)]
struct CratesIoVersionsResponse {
    versions: Vec<CratesIoVersion>,
}

#[derive(Debug, Deserialize)]
struct CratesIoVersion {
    num: String,
    features: ahash::HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CratesIoCrateResponse {
    #[serde(default)]
    versions: Vec<CratesIoVersion>,
}

#[derive(Debug, Deserialize)]
struct CratesIoVersionDetailResponse {
    version: CratesIoVersion,
}

pub async fn completion(
    _text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    position: tombi_text::Position,
    accessors: &[Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    if let Some(Accessor::Key(first)) = accessors.first() {
        if first == "workspace" {
            completion_workspace(
                _text_document,
                document_tree,
                position,
                accessors,
                _toml_version,
            )
            .await
        } else {
            completion_member(
                _text_document,
                document_tree,
                position,
                accessors,
                _toml_version,
            )
            .await
        }
    } else {
        Ok(None)
    }
}

async fn completion_workspace(
    _text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    position: tombi_text::Position,
    accessors: &[Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    use tombi_schema_store::match_accessors;
    let mut crate_name: Option<&str> = None;
    if match_accessors!(accessors, ["workspace", "dependencies", _]) {
        if let Some(Accessor::Key(crate_name)) = accessors.last() {
            return complete_crate_version(crate_name.as_str(), document_tree, accessors, position)
                .await;
        }
    } else if match_accessors!(accessors, ["workspace", "dependencies", _, "version"]) {
        if let Some(Accessor::Key(crate_name)) = accessors.get(accessors.len() - 2) {
            return complete_crate_version(crate_name.as_str(), document_tree, accessors, position)
                .await;
        }
    } else if match_accessors!(accessors, ["workspace", "dependencies", _, "features", _]) {
        if let Some(Accessor::Key(crate_name)) = accessors.get(2) {
            return complete_crate_feature(crate_name.as_str(), document_tree, accessors).await;
        }
    }
    Ok(None)
}

async fn completion_member(
    _text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    position: tombi_text::Position,
    accessors: &[Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    if match_accessors!(accessors, ["dependencies", _, "version"])
        || match_accessors!(accessors, ["dev-dependencies", _, "version"])
        || match_accessors!(accessors, ["build-dependencies", _, "version"])
    {
        if let Some(Accessor::Key(c_name)) = accessors.get(accessors.len() - 2) {
            return complete_crate_version(c_name.as_str(), document_tree, accessors, position)
                .await;
        }
    } else if match_accessors!(accessors, ["dependencies", _])
        || match_accessors!(accessors, ["dev-dependencies", _])
        || match_accessors!(accessors, ["build-dependencies", _])
    {
        if let Some(Accessor::Key(c_name)) = accessors.last() {
            return complete_crate_version(c_name.as_str(), document_tree, accessors, position)
                .await;
        }
    } else if (match_accessors!(accessors, ["dependencies", _, "features", _])
        || match_accessors!(accessors, ["dev-dependencies", _, "features", _])
        || match_accessors!(accessors, ["build-dependencies", _, "features", _]))
    {
        if let Some(Accessor::Key(crate_name)) = accessors.get(1) {
            return complete_crate_feature(crate_name.as_str(), document_tree, accessors).await;
        }
    }
    Ok(None)
}

async fn complete_crate_version(
    crate_name: &str,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
    position: tombi_text::Position,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    let version_value = match dig_accessors(document_tree, accessors) {
        Some((_, tombi_document_tree::Value::String(value_string))) => Some(value_string),
        _ => None,
    };
    if let Some(versions) = fetch_crate_versions(crate_name).await {
        let items = versions
            .into_iter()
            .sorted_by(|a, b| tombi_version_sort::version_sort(a, b))
            .rev()
            .take(100)
            .enumerate()
            .map(|(i, ver)| CompletionContent {
                label: format!("\"{ver}\""),
                kind: CompletionKind::String,
                emoji_icon: Some('🦀'),
                priority: tombi_extension::CompletionContentPriority::Custom(format!(
                    "1__cargo_{i:>03}__",
                )),
                detail: Some("Crate version".to_string()),
                documentation: None,
                filter_text: None,
                schema_url: None,
                deprecated: None,
                edit: version_value.map(|value| tombi_extension::CompletionEdit {
                    text_edit: tower_lsp::lsp_types::CompletionTextEdit::Edit(
                        tower_lsp::lsp_types::TextEdit {
                            range: tombi_text::Range::at(position).into(),
                            new_text: format!("\"{ver}\""),
                        },
                    ),
                    insert_text_format: Some(tower_lsp::lsp_types::InsertTextFormat::PLAIN_TEXT),
                    additional_text_edits: Some(vec![tower_lsp::lsp_types::TextEdit {
                        range: value.range().into(),
                        new_text: "".to_string(),
                    }]),
                }),
                preselect: None,
            })
            .collect();
        Ok(Some(items))
    } else {
        Ok(None)
    }
}

async fn complete_crate_feature(
    crate_name: &str,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    let version_string = if let Some((_, tombi_document_tree::Value::String(value_string))) =
        dig_accessors(
            document_tree,
            &accessors[..accessors.len() - 2]
                .iter()
                .chain(std::iter::once(&Accessor::Key("version".to_string())))
                .cloned()
                .collect_vec(),
        ) {
        Some(value_string.value().to_string())
    } else {
        None
    };
    let features = fetch_crate_features(crate_name, version_string.as_deref())
        .await
        .ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(format!(
                "Failed to fetch features for crate {crate_name}"
            ))
        })?;
    let already_features: Vec<String> =
        match dig_accessors(document_tree, &accessors[..accessors.len() - 1]) {
            Some((_, tombi_document_tree::Value::Array(array))) => array
                .values()
                .into_iter()
                .filter_map(|feature| {
                    if let tombi_document_tree::Value::String(feature_string) = feature {
                        Some(feature_string.value().to_string())
                    } else {
                        None
                    }
                })
                .collect(),
            _ => vec![],
        };
    let items = features
        .into_iter()
        .filter(|feat| !already_features.contains(feat))
        .sorted_by(|a, b| version_sort(a, b))
        .enumerate()
        .map(|(i, feat)| CompletionContent {
            label: format!("\"{}\"", feat),
            kind: CompletionKind::String,
            emoji_icon: Some('🦀'),
            priority: tombi_extension::CompletionContentPriority::Custom(format!(
                "1__cargo_feature_{i:>03}__",
            )),
            detail: Some("Crate feature".to_string()),
            documentation: None,
            filter_text: None,
            schema_url: None,
            deprecated: None,
            edit: None,
            preselect: None,
        })
        .collect();
    Ok(Some(items))
}

/// Fetch crate version list from crates.io API
async fn fetch_crate_versions(crate_name: &str) -> Option<Vec<String>> {
    let url = format!("https://crates.io/api/v1/crates/{}/versions", crate_name);
    let client = HttpClient::new();
    let bytes = match client
        .get_bytes(&url)
        .await
        .map_err(|e| format!("http error: {e:?}"))
    {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to fetch crate versions from {url}: {e}");
            return None;
        }
    };

    let resp: CratesIoVersionsResponse = match serde_json::from_slice(&bytes) {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("Failed to parse crate versions response: {e}");
            return None;
        }
    };
    Some(resp.versions.into_iter().map(|v| v.num).collect())
}

/// Fetch crate features list from crates.io API
async fn fetch_crate_features(crate_name: &str, version: Option<&str>) -> Option<Vec<String>> {
    let client = HttpClient::new();
    let version = if let Some(ver) = version {
        ver.to_string()
    } else {
        // fetch latest version
        let url = format!("https://crates.io/api/v1/crates/{crate_name}");
        let bytes = client.get_bytes(&url).await.ok()?;
        let resp: CratesIoCrateResponse = serde_json::from_slice(&bytes).ok()?;
        let Some(version) = resp.versions.into_iter().next().map(|v| v.num) else {
            return None;
        };
        version
    };
    let url = format!("https://crates.io/api/v1/crates/{crate_name}/{version}");
    let bytes = client.get_bytes(&url).await.ok()?;
    let resp: CratesIoVersionDetailResponse = serde_json::from_slice(&bytes).ok()?;
    let mut features: Vec<String> = resp.version.features.keys().cloned().collect();
    features.sort();
    Some(features)
}
