use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

use crate::server::state::{ServerState, State};

pub fn handle_document_symbol(
    state: State<ServerState>,
    _params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    Ok(Some(DocumentSymbolResponse::Flat(vec![])))
}
