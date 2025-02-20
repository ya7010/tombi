use tower_lsp::lsp_types::DidSaveTextDocumentParams;

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_did_save(
    backend: &Backend,
    DidSaveTextDocumentParams {
        text_document,
        text,
    }: DidSaveTextDocumentParams,
) {
    tracing::info!("handle_did_save");

    if let Some(text) = text {
        let mut document_sources = backend.document_sources.write().await;
        if let Some(document) = document_sources.get_mut(&text_document.uri) {
            document.source = text;
        }
    }
}
