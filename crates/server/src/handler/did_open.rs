use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::backend::Backend;
use crate::document::DocumentSource;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_did_open(
    backend: &Backend,
    DidOpenTextDocumentParams { text_document, .. }: DidOpenTextDocumentParams,
) {
    tracing::info!("handle_did_open");

    let version = text_document.version;
    let source = text_document.text;
    backend.document_sources.insert(
        text_document.uri.clone(),
        DocumentSource::new(source, version),
    );
}
