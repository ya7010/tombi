use itertools::Itertools;
use serde::Deserialize;
use tombi_config::TomlVersion;
use tombi_extension::CompletionContent;
use tombi_extension::CompletionKind;
use tombi_schema_store::dig_accessors;
use tombi_schema_store::Accessor;
use tombi_schema_store::HttpClient;
use tower_lsp::lsp_types::TextDocumentIdentifier;

#[derive(Debug, Deserialize)]
struct CratesIoVersionsResponse {
    versions: Vec<CratesIoVersion>,
}

#[derive(Debug, Deserialize)]
struct CratesIoVersion {
    num: String,
}

pub async fn completion(
    _text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    position: tombi_text::Position,
    accessors: &[Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    use tombi_schema_store::match_accessors;
    let mut crate_name: Option<&str> = None;
    let mut version_value = None;
    let mut should_complete = false;

    if match_accessors!(accessors, ["dependencies", _, "version"])
        || match_accessors!(accessors, ["dev-dependencies", _, "version"])
        || match_accessors!(accessors, ["build-dependencies", _, "version"])
        || match_accessors!(accessors, ["workspace", "dependencies", _, "version"])
    {
        // crate名を抽出
        if let Some(Accessor::Key(c_name)) = accessors.get(accessors.len() - 2) {
            crate_name = Some(c_name.as_str());
            should_complete = true;
            // version値の取得
            if let Some((_, tombi_document_tree::Value::String(value_string))) =
                dig_accessors(document_tree, accessors)
            {
                version_value = Some(value_string);
            }
        }
    }
    // [dependencies.*] or [workspace.dependencies.*] (e.g. serde = "1.0.0")
    else if match_accessors!(accessors, ["dependencies", _])
        || match_accessors!(accessors, ["dev-dependencies", _])
        || match_accessors!(accessors, ["build-dependencies", _])
        || match_accessors!(accessors, ["workspace", "dependencies", _])
    {
        if let Some(Accessor::Key(c_name)) = accessors.last() {
            crate_name = Some(c_name.as_str());
            should_complete = true;
            if let Some((_, tombi_document_tree::Value::String(value_string))) =
                dig_accessors(document_tree, accessors)
            {
                version_value = Some(value_string);
            }
        }
    }
    if let Some(crate_name) = crate_name {
        if should_complete {
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
                            insert_text_format: Some(
                                tower_lsp::lsp_types::InsertTextFormat::PLAIN_TEXT,
                            ),
                            additional_text_edits: Some(vec![tower_lsp::lsp_types::TextEdit {
                                range: value.range().into(),
                                new_text: "".to_string(),
                            }]),
                        }),
                        preselect: None,
                    })
                    .collect();
                return Ok(Some(items));
            }
        }
    }
    Ok(None)
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
