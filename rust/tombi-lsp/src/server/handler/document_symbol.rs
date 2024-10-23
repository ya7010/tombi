use lsp_types::{request::DocumentSymbolRequest, DocumentSymbolParams, DocumentSymbolResponse};

use super::Handler;

pub fn handle_document_symbol(
    _params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    Ok(Some(DocumentSymbolResponse::Flat(vec![])))
}

impl Handler for DocumentSymbolRequest {
    type Request = Self;

    fn handle(
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>, anyhow::Error> {
        handle_document_symbol(params)
    }
}
