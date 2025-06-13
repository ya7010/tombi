use crate::{
    handler::hover::{get_accessors_with_range, get_keys_with_range},
    Backend,
};
use itertools::{Either, Itertools};
use tombi_document_tree::TryIntoDocumentTree;
use tombi_schema_store::{dig_accessors, Accessor};
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, DocumentChanges, OneOf,
    OptionalVersionedTextDocumentIdentifier, TextDocumentEdit, TextDocumentIdentifier, TextEdit,
    WorkspaceEdit,
};

pub async fn handle_code_action(
    backend: &Backend,
    params: CodeActionParams,
) -> Result<Option<Vec<CodeActionOrCommand>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_code_action");
    tracing::trace!(?params);

    let CodeActionParams {
        text_document,
        range,
        ..
    } = params;

    let position: tombi_text::Position = range.start.into();
    let config = backend.config().await;

    if !config
        .lsp()
        .and_then(|server| server.code_action.as_ref())
        .and_then(|code_action| code_action.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.code_action.enabled` is false");
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

    let Some((keys, _)) = get_keys_with_range(&root, position, toml_version).await else {
        return Ok(None);
    };

    let Ok(document_tree) = root.try_into_document_tree(toml_version) else {
        return Ok(None);
    };

    let mut code_actions = Vec::new();
    let accessors_with_range = get_accessors_with_range(&document_tree, &keys, position);

    if let Some(code_action) =
        dot_keys_to_inline_table(&text_document, &document_tree, &accessors_with_range)
    {
        code_actions.push(code_action.into());
    }

    if code_actions.is_empty() {
        return Ok(None);
    }

    Ok(Some(code_actions))
}

fn dot_keys_to_inline_table(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors_with_range: &[(Accessor, tombi_text::Range)],
) -> Option<CodeAction> {
    if accessors_with_range.len() < 2 {
        return None;
    }

    let parent_range = accessors_with_range[accessors_with_range.len() - 2].1;
    let accessors = accessors_with_range
        .iter()
        .map(|(accessor, _)| accessor.clone())
        .collect_vec();

    let Some((accessor, value)) = dig_accessors(&document_tree, &accessors[..accessors.len() - 1])
    else {
        return None;
    };

    match (accessor, value) {
        (Accessor::Key(parent_key), tombi_document_tree::Value::Table(table))
            if table.len() == 1 =>
        {
            let (key, value) = table.key_values().iter().next().unwrap();

            Some(
                CodeAction {
                    title: "Convert Dotted keys to Inline Table".to_string(),
                    kind: Some(CodeActionKind::REFACTOR_REWRITE),
                    edit: Some(WorkspaceEdit {
                        changes: None,
                        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
                            text_document: OptionalVersionedTextDocumentIdentifier {
                                uri: text_document.clone().uri,
                                version: None,
                            },
                            edits: vec![
                                OneOf::Left(TextEdit {
                                    range: tombi_text::Range {
                                        start: parent_range.start,
                                        end: value.range().start,
                                    }
                                    .into(),
                                    new_text: format!("{} = {{ {} = ", parent_key, key.value()),
                                }),
                                OneOf::Left(TextEdit {
                                    range: tombi_text::Range::at(value.range().end).into(),
                                    new_text: " }".to_string(),
                                }),
                            ],
                        }])),
                        change_annotations: None,
                    }),
                    ..Default::default()
                }
                .into(),
            )
        }
        _ => None,
    }
}
