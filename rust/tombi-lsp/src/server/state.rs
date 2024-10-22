use lsp_types::ClientCapabilities;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub client_capabilities: ClientCapabilities,
}

impl ServerState {
    pub fn hierarchical_symbols(&self) -> bool {
        (|| -> _ {
            self.client_capabilities
                .text_document
                .as_ref()?
                .document_symbol
                .as_ref()?
                .hierarchical_document_symbol_support
        })()
        .unwrap_or_default()
    }
}
