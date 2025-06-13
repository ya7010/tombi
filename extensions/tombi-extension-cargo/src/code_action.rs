use tombi_schema_store::{dig_accessors, matches_accessors, Accessor, AccessorContext};
use tower_lsp::lsp_types::{
    CodeAction, CodeActionOrCommand, DocumentChanges, OneOf,
    OptionalVersionedTextDocumentIdentifier, TextDocumentEdit, TextDocumentIdentifier, TextEdit,
    WorkspaceEdit,
};

pub fn code_action(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
    contexts: &[AccessorContext],
) -> Result<Option<Vec<CodeActionOrCommand>>, tower_lsp::jsonrpc::Error> {
    if !text_document.uri.path().ends_with("Cargo.toml") {
        return Ok(None);
    }

    let mut code_actions = Vec::new();

    if let Some(action) =
        crate_version_code_action(text_document, document_tree, accessors, contexts)
    {
        code_actions.push(CodeActionOrCommand::CodeAction(action));
    }

    Ok(if code_actions.is_empty() {
        None
    } else {
        Some(code_actions)
    })
}

fn crate_version_code_action(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[Accessor],
    _contexts: &[AccessorContext],
) -> Option<CodeAction> {
    if matches_accessors!(accessors, ["dependencies", _])
        || matches_accessors!(accessors, ["dev-dependencies", _])
        || matches_accessors!(accessors, ["build-dependencies", _])
        || matches_accessors!(accessors, ["workspace", "dependencies", _])
    {
        if let Some((_, tombi_document_tree::Value::String(version))) =
            dig_accessors(document_tree, accessors)
        {
            return Some(CodeAction {
                title: "Convert Dependency to Table Format".to_string(),
                kind: Some(tower_lsp::lsp_types::CodeActionKind::REFACTOR_REWRITE),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: None,
                    document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
                        text_document: OptionalVersionedTextDocumentIdentifier {
                            uri: text_document.clone().uri,
                            version: None,
                        },
                        edits: vec![
                            OneOf::Left(TextEdit {
                                range: tombi_text::Range::at(version.symbol_range().start).into(),
                                new_text: "{ version = ".to_string(),
                            }),
                            OneOf::Left(TextEdit {
                                range: tombi_text::Range::at(version.symbol_range().end).into(),
                                new_text: " }".to_string(),
                            }),
                        ],
                    }])),
                    change_annotations: None,
                }),
                ..Default::default()
            });
        }
    }
    None
}
