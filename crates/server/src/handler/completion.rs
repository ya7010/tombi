use tower_lsp::lsp_types::{
    CompletionContext, CompletionParams, CompletionTriggerKind, TextDocumentPositionParams,
};

use crate::{
    backend,
    completion::{get_completion_contents, CompletionContent},
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_completion(
    backend: &backend::Backend,
    CompletionParams {
        text_document_position:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        context,
        ..
    }: CompletionParams,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");
    tracing::trace!("text_document: {:#?}", text_document);

    let config = backend.config().await;

    if !config
        .server
        .and_then(|server| server.completion)
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

    let Ok(source_schema) = &backend
        .schema_store
        .try_get_source_schema_from_url(&text_document.uri)
        .await
    else {
        tracing::debug!("schema not found: {}", text_document.uri);
        return Ok(None);
    };
    let document_sources = backend.document_sources.read().await;
    let Some(document_source) = document_sources.get(&text_document.uri) else {
        return Ok(None);
    };

    let root_schema = source_schema
        .as_ref()
        .and_then(|source_schema| source_schema.root_schema.as_ref());

    // Skip completion if the trigger character is a whitespace or if there is no schema.
    if let Some(CompletionContext {
        trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
        trigger_character: Some(trigger_character),
        ..
    }) = context
    {
        if trigger_character == "\n" {
            let pos_line = position.line as usize;
            if pos_line > 0 {
                if let Some(prev_line) = &document_source.source.lines().nth(pos_line - 1) {
                    if prev_line.trim().is_empty() || root_schema.is_none() {
                        tracing::trace!("completion skipped due to consecutive line breaks");
                        return Ok(None);
                    }
                }
            }
        }
    }

    // FIXME: Remove whitespaces, because the AST assigns the whitespace to the next section.
    //        In the future, it would be better to move the whitespace in ast_editor.
    let mut position: text::Position = position.into();
    while position.column() != 0 && position.char_at_left(&document_source.source) == Some(' ') {
        position = text::Position::new(position.line(), position.column() - 1);
    }

    let toml_version = backend.toml_version().await.unwrap_or_default();
    let Some(root) = backend
        .get_incomplete_ast(&text_document.uri, toml_version)
        .await
    else {
        return Ok(None);
    };

    Ok(Some(
        get_completion_contents(
            root,
            position,
            &schema_store::SchemaContext {
                toml_version,
                root_schema,
                sub_schema_url_map: source_schema
                    .as_ref()
                    .map(|source_schema| &source_schema.sub_schema_url_map),
                store: &backend.schema_store,
            },
        )
        .await,
    ))
}
