use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::{backend::Backend, document::DocumentSource};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_did_open(
    backend: &Backend,
    DidOpenTextDocumentParams { text_document, .. }: DidOpenTextDocumentParams,
) {
    tracing::info!("handle_did_open");
    tracing::trace!("text_document: {:#?}", text_document);

    let mut document_sources = backend.document_sources.write().await;
    document_sources.insert(
        text_document.uri,
        DocumentSource::new(text_document.text, text_document.version),
    );
}
