use tower_lsp::lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

use crate::server::backend::Backend;

pub async fn handle_document_symbol(
    backend: &Backend,
    _params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    Ok(Some(DocumentSymbolResponse::Flat(vec![])))
}
