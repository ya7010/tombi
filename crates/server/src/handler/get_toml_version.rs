use crate::backend::Backend;
use tower_lsp::lsp_types::TextDocumentIdentifier;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_get_toml_version(
    backend: &Backend,
    TextDocumentIdentifier { uri }: TextDocumentIdentifier,
) -> Result<String, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_get_toml_version");

    let schema = backend
        .schema_store
        .try_get_schema_from_url(&uri)
        .await
        .ok()
        .flatten();

    let toml_version = schema
        .as_ref()
        .map(|s| s.toml_version())
        .flatten()
        .unwrap_or(backend.toml_version().await);

    Ok(toml_version.to_string())
}
