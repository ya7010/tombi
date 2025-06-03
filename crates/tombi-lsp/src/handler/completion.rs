use itertools::Either;
use tower_lsp::lsp_types::{CompletionParams, TextDocumentPositionParams};

use crate::{
    backend,
    completion::{get_completion_contents, CompletionContent},
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_completion(
    backend: &backend::Backend,
    params: CompletionParams,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");
    tracing::trace!(?params);

    let CompletionParams {
        text_document_position:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    } = params;

    let config = backend.config().await;

    if !config
        .lsp()
        .and_then(|server| server.completion.as_ref())
        .and_then(|completion| completion.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.completion.enabled` is false");
        return Ok(None);
    }

    if !config
        .schema
        .and_then(|s| s.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`schema.enabled` is false");
        return Ok(None);
    }

    let Some(root) = backend.get_incomplete_ast(&text_document.uri).await else {
        return Ok(None);
    };

    let source_schema = backend
        .schema_store
        .resolve_source_schema_from_ast(&root, Some(Either::Left(&text_document.uri)))
        .await
        .ok()
        .flatten();

    let (toml_version, _) = backend.source_toml_version(source_schema.as_ref()).await;

    let root_schema = source_schema
        .as_ref()
        .and_then(|schema| schema.root_schema.as_ref());

    Ok(Some(
        get_completion_contents(
            root,
            position.into(),
            &text_document.uri,
            &tombi_schema_store::SchemaContext {
                toml_version,
                root_schema,
                sub_schema_url_map: source_schema
                    .as_ref()
                    .map(|schema| &schema.sub_schema_url_map),
                store: &backend.schema_store,
            },
        )
        .await,
    ))
}
