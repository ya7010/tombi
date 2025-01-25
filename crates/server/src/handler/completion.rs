use ast::{algo::ancestors_at_position, AstNode};
use dashmap::try_result::TryResult;
use document_tree::{IntoDocumentTreeResult, TryIntoDocumentTree};
use tower_lsp::lsp_types::{
    CompletionItem, CompletionParams, CompletionResponse, TextDocumentPositionParams,
};

use crate::{
    backend,
    completion::{CompletionHint, FindCompletionItems},
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
        ..
    }: CompletionParams,
) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");

    let config = backend.config().await;

    if !config
        .server
        .and_then(|s| s.completion)
        .unwrap_or_default()
        .value()
    {
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

    let items = get_completion_items(root, position.into(), document_schema, toml_version);

    Ok(Some(CompletionResponse::Array(items)))
}

fn get_completion_items(
    root: ast::Root,
    position: text::Position,
    document_schema: &schema_store::DocumentSchema,
    toml_version: config::TomlVersion,
) -> Vec<CompletionItem> {
    let mut keys: Vec<document_tree::Key> = vec![];
    let mut completion_hint = None;

    for node in ancestors_at_position(root.syntax(), position) {
        let ast_keys = if let Some(kv) = ast::KeyValue::cast(node.to_owned()) {
            kv.keys()
        } else if let Some(table) = ast::Table::cast(node.to_owned()) {
            if position < table.bracket_start().unwrap().range().start() {
                None
            } else {
                if table.contains_header(position) {
                    completion_hint = Some(CompletionHint::InTableHeader);
                }
                table.header()
            }
        } else if let Some(array_of_tables) = ast::ArrayOfTables::cast(node.to_owned()) {
            if position
                < array_of_tables
                    .double_bracket_start()
                    .unwrap()
                    .range()
                    .start()
            {
                None
            } else {
                if array_of_tables.contains_header(position) {
                    completion_hint = Some(CompletionHint::InTableHeader);
                }
                array_of_tables.header()
            }
        } else {
            continue;
        };

        let Some(ast_keys) = ast_keys else { continue };
        let mut new_keys = if ast_keys.range().contains(position) {
            let mut new_keys = Vec::with_capacity(ast_keys.keys().count());
            for key in ast_keys
                .keys()
                .take_while(|key| key.token().unwrap().range().start() <= position)
            {
                match key.try_into_document_tree(toml_version) {
                    Ok(Some(key)) => new_keys.push(key),
                    _ => return vec![],
                }
            }
            new_keys
        } else {
            let mut new_keys = Vec::with_capacity(ast_keys.keys().count());
            for key in ast_keys.keys() {
                match key.try_into_document_tree(toml_version) {
                    Ok(Some(key)) => new_keys.push(key),
                    _ => return vec![],
                }
            }
            new_keys
        };

        new_keys.extend(keys);
        keys = new_keys;
    }

    let document_tree = root.into_document_tree_result(toml_version).tree;

    let (completion_items, errors) = document_tree.find_completion_items(
        &Vec::with_capacity(0),
        document_schema.value_schema(),
        toml_version,
        position,
        &keys,
        Some(&document_schema.schema_url),
        &document_schema.definitions,
        completion_hint,
    );

    for error in errors {
        tracing::error!("error: {:?}", error);
    }

    completion_items
}
