use tower_lsp::lsp_types::{CompletionParams, CompletionResponse};

use crate::backend;

pub async fn handle_completion(
    _backend: &backend::Backend,
    _params: CompletionParams,
) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");

    Ok(None)
}
