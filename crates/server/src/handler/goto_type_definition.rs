use tower_lsp::lsp_types::request::{GotoTypeDefinitionParams, GotoTypeDefinitionResponse};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_goto_type_definition(
    _backend: &Backend,
    _params: GotoTypeDefinitionParams,
) -> Result<Option<GotoTypeDefinitionResponse>, tower_lsp::jsonrpc::Error> {
    Ok(None)
}
