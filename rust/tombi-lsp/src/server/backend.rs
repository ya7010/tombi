use dashmap::DashMap;
use tower_lsp::{
    lsp_types::{
        ClientCapabilities, DidChangeConfigurationParams, DidChangeTextDocumentParams,
        DidOpenTextDocumentParams, DocumentDiagnosticParams, DocumentDiagnosticReportResult,
        DocumentOnTypeFormattingParams, DocumentSymbolParams, DocumentSymbolResponse, Hover,
        HoverParams, InitializeParams, InitializeResult, SemanticTokensParams,
        SemanticTokensResult, TextEdit, Url,
    },
    LanguageServer,
};

use crate::document::Document;

use super::handler::{
    handle_diagnostic, handle_did_change, handle_did_change_configuration, handle_did_open,
    handle_document_symbol, handle_formatting, handle_hover, handle_initialize,
    handle_on_type_formatting, handle_semantic_tokens_full, handle_shutdown,
};

#[derive(Debug)]
pub struct Backend {
    #[allow(dead_code)]
    pub client: tower_lsp::Client,
    pub documents: DashMap<Url, Document>,
}

impl Backend {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
        }
    }
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handle_did_open(self, params).await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        handle_did_change(self, params).await
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

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>, tower_lsp::jsonrpc::Error> {
        handle_hover(params).await
    }

    async fn formatting(
        &self,
        params: tower_lsp::lsp_types::DocumentFormattingParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::TextEdit>>, tower_lsp::jsonrpc::Error> {
        handle_formatting(self, params).await
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
        handle_on_type_formatting(params).await
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
        handle_diagnostic(params).await
    }
}
