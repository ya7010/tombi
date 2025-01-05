use std::sync::Arc;

use super::handler::{
    handle_diagnostic, handle_did_change, handle_did_change_configuration, handle_did_open,
    handle_did_save, handle_document_symbol, handle_formatting, handle_hover, handle_initialize,
    handle_semantic_tokens_full, handle_shutdown,
};
use crate::{
    document::DocumentSource,
    handler::{
        handle_completion, handle_did_change_watched_files, handle_folding_range,
        handle_get_toml_version, handle_initialized, handle_update_config, handle_update_schema,
        GetTomlVersionResponse,
    },
};
use ast::AstNode;
use config::{Config, TomlVersion};
use dashmap::DashMap;
use tokio::sync::RwLock;
use tower_lsp::{
    lsp_types::{
        CompletionParams, CompletionResponse, DidChangeConfigurationParams,
        DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, DocumentDiagnosticParams, DocumentDiagnosticReportResult,
        DocumentSymbolParams, DocumentSymbolResponse, FoldingRange, FoldingRangeParams, Hover,
        HoverParams, InitializeParams, InitializeResult, InitializedParams, SemanticTokensParams,
        SemanticTokensResult, TextDocumentIdentifier, Url,
    },
    LanguageServer,
};

#[derive(Debug)]
pub struct Backend {
    #[allow(dead_code)]
    pub client: tower_lsp::Client,
    pub document_sources: DashMap<Url, DocumentSource>,
    config: Arc<RwLock<Config>>,
    pub schema_store: schema_store::SchemaStore,
}

impl Backend {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self {
            client,
            document_sources: Default::default(),
            config: Arc::new(RwLock::new(match config::load() {
                Ok(config) => config,
                Err(err) => {
                    tracing::error!("{err}");
                    Config::default()
                }
            })),
            schema_store: schema_store::SchemaStore::new(),
        }
    }

    pub fn get_ast(&self, uri: &Url, toml_version: TomlVersion) -> Option<ast::Root> {
        if let Some(document_info) = self.document_sources.get(uri) {
            let p = parser::parse(&document_info.source, toml_version);
            if !p.errors().is_empty() {
                return None;
            }

            return ast::Root::cast(p.into_syntax_node());
        }
        None
    }

    pub async fn config(&self) -> Config {
        self.config.read().await.clone()
    }

    pub async fn update_workspace_config(&self, workspace_config_url: Url, config: Config) {
        tracing::info!("Updated workspace config: {workspace_config_url}");

        *self.config.write().await = config;
    }

    pub async fn toml_version(&self) -> Option<TomlVersion> {
        self.config.read().await.toml_version
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        params: InitializeParams,
    ) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
        handle_initialize(params).await
    }

    async fn initialized(&self, params: InitializedParams) {
        handle_initialized(self, params).await
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

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        handle_did_change_watched_files(params).await
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        handle_did_save(self, params).await
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        handle_did_change_configuration(params).await
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
        handle_completion(self, params).await
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
        handle_hover(self, params).await
    }

    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>, tower_lsp::jsonrpc::Error> {
        handle_folding_range(self, params).await
    }

    async fn formatting(
        &self,
        params: tower_lsp::lsp_types::DocumentFormattingParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::TextEdit>>, tower_lsp::jsonrpc::Error> {
        handle_formatting(self, params).await
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
        handle_diagnostic(self, params).await
    }
}

impl Backend {
    pub async fn get_toml_version(
        &self,
        params: TextDocumentIdentifier,
    ) -> Result<GetTomlVersionResponse, tower_lsp::jsonrpc::Error> {
        handle_get_toml_version(self, params).await
    }

    pub async fn update_schema(
        &self,
        params: TextDocumentIdentifier,
    ) -> Result<bool, tower_lsp::jsonrpc::Error> {
        handle_update_schema(self, params).await
    }

    pub async fn update_config(
        &self,
        params: TextDocumentIdentifier,
    ) -> Result<bool, tower_lsp::jsonrpc::Error> {
        handle_update_config(self, params).await
    }
}
