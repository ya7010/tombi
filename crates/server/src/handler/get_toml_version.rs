use config::TomlVersion;
use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_get_toml_version(
    backend: &Backend,
    TextDocumentIdentifier { uri }: TextDocumentIdentifier,
) -> Result<GetTomlVersionResponse, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_get_toml_version");

    let source_schema = backend
        .schema_store
        .try_get_source_schema_from_url(&uri)
        .await
        .ok()
        .flatten();

    let (toml_version, source) = source_schema
        .as_ref()
        .and_then(|source_schema| source_schema.root_schema.as_ref())
        .and_then(|document_schema| {
            document_schema
                .toml_version()
                .map(|toml_version| (toml_version, "schema"))
        })
        .unwrap_or(match backend.toml_version().await {
            Some(toml_version) => (toml_version, "config"),
            None => (TomlVersion::default(), "default"),
        });

    Ok(GetTomlVersionResponse {
        toml_version,
        source,
    })
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTomlVersionResponse {
    pub toml_version: TomlVersion,
    pub source: &'static str,
}
