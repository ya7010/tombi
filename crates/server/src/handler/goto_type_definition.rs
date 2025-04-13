use tower_lsp::lsp_types::request::{GotoTypeDefinitionParams, GotoTypeDefinitionResponse};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_goto_type_definition(
    backend: &Backend,
    params: GotoTypeDefinitionParams,
) -> Result<Option<GotoTypeDefinitionResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_goto_type_definition");
    tracing::trace!(?params);

    let GotoTypeDefinitionParams {
        text_document_position_params:
            tower_lsp::lsp_types::TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    } = params;

    let config = backend.config().await;

    if !config
        .server
        .and_then(|server| server.goto_type_definition)
        .and_then(|goto_type_definition| goto_type_definition.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.goto_type_definition.enabled` is false");
        return Ok(None);
    }

    let (toml_version, _) = backend.text_document_toml_version(&text_document.uri).await;
    let Some(root) = backend.get_incomplete_ast(&text_document.uri).await else {
        return Ok(None);
    };

    let source_schema = backend
        .schema_store
        .try_get_source_schema_from_ast(&root, Some(Either::Left(&text_document.uri)))
        .await
        .ok()
        .flatten();

    Ok(None)
}
