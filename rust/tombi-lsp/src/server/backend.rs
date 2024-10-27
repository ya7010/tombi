use dashmap::DashMap;
use tower_lsp::{
    lsp_types::{
        DidChangeConfigurationParams, DocumentHighlight, DocumentHighlightParams,
        DocumentSymbolParams, DocumentSymbolResponse, InitializeParams, InitializeResult,
        SemanticTokensParams, SemanticTokensResult, Url,
    },
    LanguageServer,
};

use super::handler::{
    handle_did_change_configuration, handle_document_symbol, handle_formatting, handle_initialize,
    handle_semantic_tokens_full, handle_shutdown,
};

#[derive(Debug)]
pub struct Backend {
    pub client: tower_lsp::Client,
    pub _file_map: DashMap<Url, String>,
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

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        handle_did_change_configuration(params).await
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
        handle_semantic_tokens_full(self, params).await
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
        handle_document_symbol(self, params).await
    }

    async fn formatting(
        &self,
        params: tower_lsp::lsp_types::DocumentFormattingParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::TextEdit>>, tower_lsp::jsonrpc::Error> {
        handle_formatting(self, params).await
    }
}
