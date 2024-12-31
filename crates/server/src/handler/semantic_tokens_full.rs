use crate::semantic_tokens::AppendSemanticTokens;
use crate::toml;
use crate::{backend::Backend, semantic_tokens::SemanticTokensBuilder};
use ast::AstNode;
use config::TomlVersion;
use tower_lsp::lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_semantic_tokens_full(
    _backend: &Backend,
    SemanticTokensParams { text_document, .. }: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_semantic_tokens_full");

    let source = toml::try_load(&text_document.uri)?;

    let p = parser::parse(&source, TomlVersion::default());
    let Some(ast) = ast::Root::cast(p.into_syntax_node()) else {
        return Ok(None);
    };

    let mut tokens_builder = SemanticTokensBuilder::new();
    ast.append_semantic_tokens(&mut tokens_builder);
    let tokens = tokens_builder.build();

    tracing::trace!("SemanticTokens: {tokens:#?}");

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
    })))
}
