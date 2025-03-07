use std::sync::Arc;

use ahash::AHashMap;
use config::{Config, TomlVersion};
use diagnostic::{Diagnostic, SetDiagnostics};
use document_tree::TryIntoDocumentTree;
use syntax::SyntaxNode;
use tower_lsp::{
    lsp_types::{
        CompletionParams, CompletionResponse, DidChangeConfigurationParams,
        DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentDiagnosticParams,
        DocumentDiagnosticReportResult, DocumentSymbolParams, DocumentSymbolResponse, FoldingRange,
        FoldingRangeParams, Hover, HoverParams, InitializeParams, InitializeResult,
        InitializedParams, SemanticTokensParams, SemanticTokensResult, TextDocumentIdentifier, Url,
    },
    LanguageServer,
};

use super::handler::{
    handle_diagnostic, handle_did_change, handle_did_change_configuration, handle_did_open,
    handle_did_save, handle_document_symbol, handle_formatting, handle_hover, handle_initialize,
    handle_semantic_tokens_full, handle_shutdown,
};
use crate::{
    document::DocumentSource,
    handler::{
        handle_completion, handle_did_change_watched_files, handle_did_close, handle_folding_range,
        handle_get_toml_version, handle_initialized, handle_update_config, handle_update_schema,
        GetTomlVersionResponse,
    },
};

#[derive(Debug)]
pub struct Backend {
    #[allow(dead_code)]
    pub client: tower_lsp::Client,
    pub document_sources: Arc<tokio::sync::RwLock<AHashMap<Url, DocumentSource>>>,
    pub config_dirpath: Option<std::path::PathBuf>,
    config: Arc<tokio::sync::RwLock<Config>>,
    pub schema_store: schema_store::SchemaStore,
}

impl Backend {
    #[inline]
    pub fn new(client: tower_lsp::Client, offline: bool) -> Self {
        let (config, config_dirpath) = match config::load_with_path() {
            Ok((config, config_dirpath)) => (config, config_dirpath),
            Err(err) => {
                tracing::error!("{err}");
                (Config::default(), None)
            }
        };
        Self {
            client,
            document_sources: Default::default(),
            config_dirpath,
            config: Arc::new(tokio::sync::RwLock::new(config)),
            schema_store: schema_store::SchemaStore::new(offline),
        }
    }

    #[inline]
    async fn get_parsed(
        &self,
        uri: &Url,
        toml_version: TomlVersion,
    ) -> Option<parser::Parsed<SyntaxNode>> {
        let document_source = self.document_sources.read().await;
        let document_info = match document_source.get(uri) {
            Some(document_info) => document_info,
            None => {
                tracing::warn!("document not found: {}", uri);
                return None;
            }
        };

        Some(parser::parse(&document_info.source, toml_version))
    }

    #[inline]
    pub async fn get_incomplete_ast(
        &self,
        uri: &Url,
        toml_version: TomlVersion,
    ) -> Option<ast::Root> {
        self.get_parsed(uri, toml_version)
            .await?
            .cast::<ast::Root>()
            .map(|root| root.tree())
    }

    #[inline]
    pub async fn try_get_ast(
        &self,
        uri: &Url,
        toml_version: TomlVersion,
    ) -> Option<Result<ast::Root, Vec<Diagnostic>>> {
        let Some(p) = self
            .get_parsed(uri, toml_version)
            .await?
            .cast::<ast::Root>()
        else {
            unreachable!("TOML Root node is always a valid AST node even if source is empty.")
        };

        if p.errors().is_empty() {
            Some(Ok(p.tree()))
        } else {
            let mut diagnostics = Vec::with_capacity(p.errors().len());
            p.errors().iter().for_each(|error| {
                error.set_diagnostics(&mut diagnostics);
            });

            Some(Err(diagnostics))
        }
    }

    #[inline]
    pub async fn get_incomplete_document_tree(
        &self,
        uri: &Url,
        toml_version: TomlVersion,
    ) -> Option<document_tree::DocumentTree> {
        self.get_incomplete_ast(uri, toml_version)
            .await?
            .try_into_document_tree(toml_version)
            .ok()
    }

    #[inline]
    pub async fn config(&self) -> Config {
        self.config.read().await.clone()
    }

    #[inline]
    pub async fn update_workspace_config(&self, workspace_config_url: Url, config: Config) {
        tracing::info!("Updated workspace config: {workspace_config_url}");

        *self.config.write().await = config;
    }

    #[inline]
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
        handle_shutdown().await
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handle_did_open(self, params).await
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        handle_did_close(self, params).await
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
        handle_completion(self, params).await.map(|response| {
            response.map(|completion_content| {
                CompletionResponse::Array(completion_content.into_iter().map(Into::into).collect())
            })
        })
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
        handle_hover(self, params)
            .await
            .map(|response| response.map(|hover_content| hover_content.into()))
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
