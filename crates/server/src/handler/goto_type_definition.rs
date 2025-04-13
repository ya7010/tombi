use document_tree::IntoDocumentTreeAndErrors;
use itertools::Either;
use schema_store::SchemaContext;
use tower_lsp::lsp_types::{
    request::{GotoTypeDefinitionParams, GotoTypeDefinitionResponse},
    Location, LocationLink,
};

use crate::{backend::Backend, handler::hover::get_hover_range, hover::get_hover_content};

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
                ..
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

    let position = position.into();
    let Some(root) = backend.get_incomplete_ast(&text_document.uri).await else {
        return Ok(None);
    };

    let source_schema = backend
        .schema_store
        .try_get_source_schema_from_ast(&root, Some(Either::Left(&text_document.uri)))
        .await
        .ok()
        .flatten();

    let (toml_version, _) = backend.source_toml_version(source_schema.as_ref()).await;

    let Some((keys, range)) = get_hover_range(&root, position, toml_version).await else {
        return Ok(None);
    };

    if keys.is_empty() && range.is_none() {
        return Ok(None);
    }

    let document_tree = root.into_document_tree_and_errors(toml_version).tree;

    Ok(get_hover_content(
        &document_tree,
        position,
        &keys,
        &SchemaContext {
            toml_version,
            root_schema: source_schema.as_ref().and_then(|s| s.root_schema.as_ref()),
            sub_schema_url_map: source_schema.as_ref().map(|s| &s.sub_schema_url_map),
            store: &backend.schema_store,
        },
    )
    .await
    .and_then(|content| {
        content.schema_url.map(|schema_url| {
            GotoTypeDefinitionResponse::Scalar(Location {
                uri: schema_url.into(),
                range: text::Range::default().into(),
            })
        })
    }))
}
