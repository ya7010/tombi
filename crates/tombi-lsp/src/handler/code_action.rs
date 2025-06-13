use crate::Backend;
use itertools::{Either, Itertools};
use tombi_ast::{algo::ancestors_at_position, AstNode};
use tombi_document_tree::{TableKind, TryIntoDocumentTree};
use tombi_schema_store::{dig_accessors, get_accessors, Accessor};
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

    let Some((keys, contexts)) =
        get_completion_keys_with_context(&root, position, toml_version).await
    else {
        return Ok(None);
    };

    let Ok(document_tree) = root.try_into_document_tree(toml_version) else {
        return Ok(None);
    };

    let accessors = get_accessors(&document_tree, &keys, position);

    let mut code_actions = Vec::new();

    if let Some(code_action) =
        dot_keys_to_inline_table(&text_document, &document_tree, &accessors, contexts)
    {
        code_actions.push(code_action.into());
    }
    if let Some(code_action) =
        inline_table_to_dot_keys(&text_document, &document_tree, &accessors, contexts)
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
    accessors: &[Accessor],
    contexts: Vec<KeyContext>,
) -> Option<CodeAction> {
    if accessors.len() < 2 {
        return None;
    }
    debug_assert!(accessors.len() == contexts.len());
    let parent_context = &contexts[accessors.len() - 2];

    let Some((accessor, value)) = dig_accessors(&document_tree, &accessors[..accessors.len() - 1])
    else {
        return None;
    };

    match (accessor, value) {
        (Accessor::Key(parent_key), tombi_document_tree::Value::Table(table))
            if table.len() == 1
                && matches!(parent_context.kind, KeyKind::Dotted | KeyKind::KeyValue) =>
        {
            let (key, value) = table.key_values().iter().next().unwrap();

            if table.kind() == TableKind::InlineTable {
                return None;
            }

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
                                        start: parent_context.range.start,
                                        end: value.range().start,
                                    }
                                    .into(),
                                    new_text: format!(
                                        "{} = {{ {}{}",
                                        parent_key,
                                        key.value(),
                                        if table.kind() == TableKind::KeyValue {
                                            " = "
                                        } else {
                                            "."
                                        }
                                    ),
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

fn inline_table_to_dot_keys(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
    contexts: Vec<KeyContext>,
) -> Option<CodeAction> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyKind {
    Header,
    Dotted,
    KeyValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyContext {
    kind: KeyKind,
    range: tombi_text::Range,
}

async fn get_completion_keys_with_context(
    root: &tombi_ast::Root,
    position: tombi_text::Position,
    toml_version: tombi_config::TomlVersion,
) -> Option<(Vec<tombi_document_tree::Key>, Vec<KeyContext>)> {
    let mut keys_vec = vec![];
    let mut key_contexts = vec![];

    for node in ancestors_at_position(root.syntax(), position) {
        if let Some(kv) = tombi_ast::KeyValue::cast(node.to_owned()) {
            let keys = kv.keys()?;
            let keys = if keys.range().contains(position) {
                keys.keys()
                    .take_while(|key| key.token().unwrap().range().start <= position)
                    .collect_vec()
            } else {
                keys.keys().collect_vec()
            };
            for (i, key) in keys.into_iter().rev().enumerate() {
                match key.try_into_document_tree(toml_version) {
                    Ok(Some(key_dt)) => {
                        let kind = if i == 0 {
                            KeyKind::KeyValue
                        } else {
                            KeyKind::Dotted
                        };
                        keys_vec.push(key_dt.clone());
                        key_contexts.push(KeyContext {
                            kind,
                            range: key_dt.range(),
                        });
                    }
                    _ => return None,
                }
            }
        } else if let Some(table) = tombi_ast::Table::cast(node.to_owned()) {
            if let Some(header) = table.header() {
                for key in header.keys().rev() {
                    match key.try_into_document_tree(toml_version) {
                        Ok(Some(key_dt)) => {
                            keys_vec.push(key_dt.clone());
                            key_contexts.push(KeyContext {
                                kind: KeyKind::Header,
                                range: key_dt.range(),
                            });
                        }
                        _ => return None,
                    }
                }
            }
        } else if let Some(array_of_table) = tombi_ast::ArrayOfTable::cast(node.to_owned()) {
            if let Some(header) = array_of_table.header() {
                for key in header.keys().rev() {
                    match key.try_into_document_tree(toml_version) {
                        Ok(Some(key_dt)) => {
                            keys_vec.push(key_dt.clone());
                            key_contexts.push(KeyContext {
                                kind: KeyKind::Header,
                                range: key_dt.range(),
                            });
                        }
                        _ => return None,
                    }
                }
            }
        }
    }

    if keys_vec.is_empty() {
        return None;
    }
    Some((
        keys_vec.into_iter().rev().collect(),
        key_contexts.into_iter().rev().collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tombi_config::TomlVersion;
    use tombi_parser::parse;
    use tombi_text::Position;

    #[tokio::test]
    async fn test_get_completion_keys_with_context_simple_keyvalue() {
        let src = r#"foo = 1\nbar = 2\n"#;
        let root =
            tombi_ast::Root::cast(parse(src, TomlVersion::V1_0_0).into_syntax_node()).unwrap();
        let pos = Position::new(0, 2); // somewhere in 'foo'
        let toml_version = TomlVersion::V1_0_0;
        let result = get_completion_keys_with_context(&root, pos, toml_version).await;
        assert!(result.is_some());
        let (keys, contexts) = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].kind, KeyKind::KeyValue);
    }

    #[tokio::test]
    async fn test_get_completion_keys_with_context_table_header() {
        let src = r#"[table]\nfoo = 1\n"#;
        let root =
            tombi_ast::Root::cast(parse(src, TomlVersion::V1_0_0).into_syntax_node()).unwrap();
        let pos = Position::new(0, 2); // somewhere in 'table'
        let toml_version = TomlVersion::V1_0_0;
        let result = get_completion_keys_with_context(&root, pos, toml_version).await;
        assert!(result.is_some());
        let (keys, contexts) = result.unwrap();
        assert!(!keys.is_empty());
        assert!(contexts.iter().any(|c| c.kind == KeyKind::Header));
    }

    #[tokio::test]
    async fn test_get_completion_keys_with_context_empty() {
        let src = r#"# just a comment\n"#;
        let root =
            tombi_ast::Root::cast(parse(src, TomlVersion::V1_0_0).into_syntax_node()).unwrap();
        let pos = Position::new(0, 0);
        let toml_version = TomlVersion::V1_0_0;
        let result = get_completion_keys_with_context(&root, pos, toml_version).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_completion_keys_with_context_simple_keyvalue_range() {
        let src = "foo = 1\nbar = 2\n";
        let root =
            tombi_ast::Root::cast(parse(src, TomlVersion::V1_0_0).into_syntax_node()).unwrap();
        let pos = Position::new(0, 2); // somewhere in 'foo'
        let toml_version = TomlVersion::V1_0_0;
        let result = get_completion_keys_with_context(&root, pos, toml_version).await;
        assert!(result.is_some());
        let (keys, contexts) = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(contexts.len(), 1);
        // 'foo' の範囲
        let expected_range = keys[0].range();
        assert_eq!(contexts[0].range, expected_range);
    }

    #[tokio::test]
    async fn test_get_completion_keys_with_context_table_header_range() {
        let src = "[table]\nfoo = 1\n";
        let root =
            tombi_ast::Root::cast(parse(src, TomlVersion::V1_0_0).into_syntax_node()).unwrap();
        let pos = Position::new(0, 2); // somewhere in 'table'
        let toml_version = TomlVersion::V1_0_0;
        let result = get_completion_keys_with_context(&root, pos, toml_version).await;
        assert!(result.is_some());
        let (keys, contexts) = result.unwrap();
        assert!(!keys.is_empty());

        for (key, ctx) in keys.iter().zip(contexts.iter()) {
            assert_eq!(ctx.range, key.range());
        }
    }
}
