use tower_lsp::lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};

use crate::{
    backend::Backend,
    semantic_tokens::{AppendSemanticTokens, SemanticTokensBuilder},
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_semantic_tokens_full(
    backend: &Backend,
    SemanticTokensParams { text_document, .. }: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_semantic_tokens_full");

    let toml_version = backend.toml_version().await.unwrap_or_default();
    let Some(Ok(root)) = backend.try_get_ast(&text_document.uri, toml_version).await else {
        return Ok(None);
    };

    let mut tokens_builder = SemanticTokensBuilder::new();
    root.append_semantic_tokens(&mut tokens_builder);
    let tokens = tokens_builder.build();

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
    })))
}
