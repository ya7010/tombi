use tower_lsp::lsp_types::DidCloseTextDocumentParams;

use crate::Backend;

pub async fn handle_did_close(
    backend: &Backend,
    DidCloseTextDocumentParams { text_document }: DidCloseTextDocumentParams,
) {
    tracing::info!("handle_did_close");
    tracing::trace!("text_document: {:#?}", text_document);

    let mut document_sources = backend.document_sources.write().await;
    document_sources.remove(&text_document.uri);
}
