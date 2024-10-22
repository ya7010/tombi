use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

use crate::server::state::ServerState;

pub fn handle_document_symbol(
    state: ServerState,
    params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    let _p = tracing::info_span!("handle_document_symbol").entered();
    tracing::info!("params: {:?}", params);
    if state.hierarchical_symbols() {
        Ok(Some(DocumentSymbolResponse::Nested(vec![])))
    } else {
        Ok(Some(DocumentSymbolResponse::Flat(vec![])))
    }
}
