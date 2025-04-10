use config::TomlVersion;
use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_get_toml_version(
    backend: &Backend,
    params: TextDocumentIdentifier,
) -> Result<GetTomlVersionResponse, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_get_toml_version");
    tracing::trace!(?params);

    let TextDocumentIdentifier { uri } = params;

    let (toml_version, source) = backend.text_document_toml_version(&uri).await;

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
