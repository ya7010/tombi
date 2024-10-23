use std::ops::{Deref, DerefMut};

use lsp_types::ClientCapabilities;

#[derive(Debug, Default, Clone, Copy)]
pub struct State<S>(pub S);

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for State<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
