use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::{document::Document, server::backend::Backend};

pub async fn handle_did_open(backend: &Backend, params: DidOpenTextDocumentParams) {
    tracing::info!("handle_did_open");

    let uri = params.text_document.uri.clone();
    let source = params.text_document.text;

    backend.documents.insert(uri, Document::new(source));
}
