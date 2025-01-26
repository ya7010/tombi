use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_did_open(
    backend: &Backend,
    DidOpenTextDocumentParams { text_document, .. }: DidOpenTextDocumentParams,
) {
    tracing::info!("handle_did_open");

    backend.insert_source(text_document.uri, text_document.text, text_document.version);
}
