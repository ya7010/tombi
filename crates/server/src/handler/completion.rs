use ast::{algo::ancestors_at_position, AstNode};
use dashmap::try_result::TryResult;
use tower_lsp::lsp_types::{
    self, CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse,
    TextDocumentPositionParams,
};

use crate::backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_completion(
    backend: &backend::Backend,
    CompletionParams {
        text_document_position:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    }: CompletionParams,
) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");

    let config = backend.config().await;

    if !config.server.and_then(|s| s.completion).unwrap_or_default() {
        tracing::debug!("`server.completion` is false");
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

    let uri = &text_document.uri;
    let document_info = match backend.document_sources.try_get_mut(uri) {
        TryResult::Present(document_info) => document_info,
        TryResult::Absent => {
            tracing::warn!("document not found: {}", uri);
            return Ok(None);
        }
        TryResult::Locked => {
            tracing::warn!("document is locked: {}", uri);
            return Ok(None);
        }
    };

    let Ok(Some(document_schema)) = &backend
        .schema_store
        .try_get_schema_from_url(&text_document.uri)
        .await
    else {
        tracing::debug!("schema not found: {}", text_document.uri);
        return Ok(None);
    };

    let toml_version = backend.toml_version().await.unwrap_or_default();

    let Some(root) =
        ast::Root::cast(parser::parse(&document_info.source, toml_version).into_syntax_node())
    else {
        tracing::warn!("failed to parse document: {}", uri);
        return Ok(None);
    };

    let items = get_completion_items(&root, position.into(), document_schema, toml_version);

    Ok(Some(CompletionResponse::Array(items)))
}

fn get_completion_items(
    root: &ast::Root,
    position: text::Position,
    document_schema: &schema_store::DocumentSchema,
    toml_version: config::TomlVersion,
) -> Vec<CompletionItem> {
    let mut items = vec![];

    let mut accessors: Vec<schema_store::Accessor> = vec![];
    for node in ancestors_at_position(root.syntax(), position) {
        if let Some(kv) = ast::KeyValue::cast(node.to_owned()) {
            if let Some(keys) = kv.keys() {
                for key in keys.keys() {
                    if key.syntax().range().end() < position {
                        accessors.push(schema_store::Accessor::Key(
                            key.try_to_raw_text(toml_version)
                                .unwrap_or(key.to_string())
                                .into(),
                        ));
                    }
                }
            }
        } else if let Some(array_of_tables) = ast::ArrayOfTables::cast(node.to_owned()) {
            if let Some(header) = array_of_tables.header() {
                let mut header_keys = vec![];
                for key in header.keys() {
                    if key.syntax().range().end() < position {
                        header_keys.push(schema_store::Accessor::Key(key.to_string().into()));
                    }
                }

                header_keys.extend(accessors);
                accessors = header_keys;
            }
        } else if let Some(table) = ast::Table::cast(node.to_owned()) {
            if let Some(header) = table.header() {
                let mut header_keys = vec![];
                for key in header.keys() {
                    if key.syntax().range().end() < position {
                        header_keys.push(schema_store::Accessor::Key(
                            key.try_to_raw_text(toml_version)
                                .unwrap_or(key.to_string())
                                .into(),
                        ));
                    }
                }

                header_keys.extend(accessors);
                accessors = header_keys;
            }
        }
    }

    if let Ok(mut properties) = document_schema.properties.write() {
        if accessors.is_empty() {
            for (key, value_schema) in properties.iter_mut() {
                let value_schema = match value_schema.resolve(document_schema) {
                    Ok(value_schema) => value_schema,
                    Err(err) => {
                        tracing::warn!("failed to resolve {key} schema. {err}");
                        continue;
                    }
                };
                let completion_item = CompletionItem {
                    label: key.to_string(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: value_schema.title().map(ToString::to_string),
                    documentation: value_schema.description().map(|description| {
                        lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: description.to_string(),
                        })
                    }),
                    ..Default::default()
                };

                items.push(completion_item);
            }
        } else {
            tracing::warn!("failed to acquire the DocumentSchema write lock");
        }
    }

    items
}
