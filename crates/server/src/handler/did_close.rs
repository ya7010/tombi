use tower_lsp::lsp_types::{DidCloseTextDocumentParams, TextDocumentIdentifier};

use crate::Backend;

pub async fn handle_did_close(
    backend: &Backend,
    DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri, .. },
    }: DidCloseTextDocumentParams,
) {
    backend.remove_document_source(&uri);
}
