use tower_lsp::lsp_types::DidSaveTextDocumentParams;

use crate::server::backend::Backend;

pub async fn handle_did_save(
    _backend: &Backend,
    DidSaveTextDocumentParams { .. }: DidSaveTextDocumentParams,
) {
    tracing::info!("handle_did_save");
}
