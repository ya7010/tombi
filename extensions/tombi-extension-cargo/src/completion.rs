use itertools::Itertools;
use serde::Deserialize;
use tombi_config::TomlVersion;
use tombi_extension::CompletionContent;
use tombi_extension::CompletionKind;
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
    _document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    // Determine if this is a version completion for [dependencies], [workspace.dependencies], etc.
    // 1. [dependencies, crate, version] pattern
    // 2. [workspace, dependencies, crate, version] pattern
    // 3. [dependencies, crate] pattern (e.g. serde = "1.0.0")
    // 4. [workspace, dependencies, crate] pattern
    let mut crate_name: Option<&str> = None;
    let mut should_complete = false;

    if accessors.len() >= 3 {
        // [dependencies, crate, version] etc.
        if let (Some(dep_table), Some(c_name), Some(version_key)) = (
            accessors.get(accessors.len() - 3).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 2).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 1).and_then(|a| a.as_key()),
        ) {
            let dep_tables = ["dependencies", "dev-dependencies", "build-dependencies"];
            if dep_tables.contains(&dep_table) && version_key == "version" {
                crate_name = Some(c_name);
                should_complete = true;
            }
        }
    }
    if !should_complete && accessors.len() >= 4 {
        // [workspace, dependencies, crate, version]
        if let (Some(ws), Some(dep_table), Some(c_name), Some(version_key)) = (
            accessors.get(accessors.len() - 4).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 3).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 2).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 1).and_then(|a| a.as_key()),
        ) {
            if ws == "workspace" && dep_table == "dependencies" && version_key == "version" {
                crate_name = Some(c_name);
                should_complete = true;
            }
        }
    }
    if !should_complete && accessors.len() >= 2 {
        // Pattern like serde = "1.0.0": [dependencies, crate] or [workspace, dependencies, crate]
        if let (Some(dep_table), Some(c_name)) = (
            accessors.get(accessors.len() - 2).and_then(|a| a.as_key()),
            accessors.get(accessors.len() - 1).and_then(|a| a.as_key()),
        ) {
            let dep_tables = ["dependencies", "dev-dependencies", "build-dependencies"];
            if dep_tables.contains(&dep_table) {
                crate_name = Some(c_name);
                should_complete = true;
            }
        }
        if !should_complete && accessors.len() >= 3 {
            if let (Some(ws), Some(dep_table), Some(c_name)) = (
                accessors.get(accessors.len() - 3).and_then(|a| a.as_key()),
                accessors.get(accessors.len() - 2).and_then(|a| a.as_key()),
                accessors.get(accessors.len() - 1).and_then(|a| a.as_key()),
            ) {
                if ws == "workspace" && dep_table == "dependencies" {
                    crate_name = Some(c_name);
                    should_complete = true;
                }
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
                        label: ver,
                        kind: CompletionKind::String,
                        emoji_icon: Some('🦀'),
                        priority: tombi_extension::CompletionContentPriority::Custom(format!(
                            "0__cargo_{i:>3}__",
                        )),
                        detail: Some("Crate version".to_string()),
                        documentation: None,
                        filter_text: None,
                        schema_url: None,
                        deprecated: None,
                        edit: None,
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
