use dashmap::DashMap;
use tower_lsp::{
    lsp_types::{InitializeParams, InitializeResult},
    LanguageServer,
};

use super::handler::{
    handle_document_symbol, handle_formatting, handle_initialize, handle_shutdown,
};

#[derive(Debug)]
pub struct Backend {
    pub client: tower_lsp::Client,
    pub file_map: DashMap<String, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        params: InitializeParams,
    ) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
        handle_initialize(params)
    }

    async fn shutdown(&self) -> Result<(), tower_lsp::jsonrpc::Error> {
        handle_shutdown()
    }

    async fn document_symbol(
        &self,
        params: tower_lsp::lsp_types::DocumentSymbolParams,
    ) -> Result<Option<tower_lsp::lsp_types::DocumentSymbolResponse>, tower_lsp::jsonrpc::Error>
    {
        handle_document_symbol(self, params).await
    }

    async fn formatting(
        &self,
        params: tower_lsp::lsp_types::DocumentFormattingParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::TextEdit>>, tower_lsp::jsonrpc::Error> {
        handle_formatting(params).await
    }
}
