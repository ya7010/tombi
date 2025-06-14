use tombi_lsp::code_action::CodeActionRefactorRewriteName;

macro_rules! test_code_action_refactor_rewrite {
    (
        #[tokio::test]
        async fn $name:ident(
            $source:expr,
            $select:expr,
            $schema_file_path:expr$(,)?
        ) -> Ok($expected:expr);
    ) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use std::io::Write;
            use tombi_lsp::handler::handle_code_action;
            use tombi_lsp::handler::handle_did_open;
            use tombi_lsp::Backend;
            use tower_lsp::lsp_types::{CodeActionParams, TextDocumentIdentifier, Url};
            use tower_lsp::lsp_types::{DidOpenTextDocumentParams, TextDocumentItem};
            use tower_lsp::LspService;

            tombi_test_lib::init_tracing();

            let (service, _) = LspService::new(|client| {
                Backend::new(client, &tombi_lsp::backend::Options::default())
            });
            let backend = service.inner();

            if let Some(schema_file_path) = $schema_file_path.as_ref() {
                let schema_url = tombi_schema_store::SchemaUrl::from_file_path(schema_file_path)
                    .expect(&format!(
                        "failed to convert schema path to URL: {}",
                        schema_file_path.display()
                    ));
                backend
                    .schema_store
                    .load_schemas(
                        &[tombi_config::Schema::Root(tombi_config::RootSchema {
                            toml_version: None,
                            path: schema_url.to_string(),
                            include: vec!["*.toml".to_string()],
                        })],
                        None,
                    )
                    .await;
            }

            let temp_file = tempfile::NamedTempFile::with_suffix_in(
                ".toml",
                std::env::current_dir().expect("failed to get current directory"),
            )?;

            let mut toml_text = textwrap::dedent($source).trim().to_string();
            let Some(index) = toml_text.find("█") else {
                return Err(
                    "failed to find code action position marker (█) in the test data".into(),
                );
            };
            toml_text.remove(index);
            tracing::debug!(?toml_text, "test toml text");
            tracing::debug!(?index, "test toml text index");
            temp_file.as_file().write_all(toml_text.as_bytes())?;
            let toml_file_url = Url::from_file_path(temp_file.path())
                .map_err(|_| "failed to convert temporary file path to URL")?;

            handle_did_open(
                backend,
                DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: toml_file_url.clone(),
                        language_id: "toml".to_string(),
                        version: 0,
                        text: toml_text.clone(),
                    },
                },
            )
            .await;

            let params = CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: toml_file_url.clone(),
                },
                range: tombi_text::Range::at(
                    (tombi_text::Position::default()
                        + tombi_text::RelativePosition::of(&toml_text[..index])),
                )
                .into(),
                context: Default::default(),
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            };

            let actions = handle_code_action(backend, params)
                .await?
                .unwrap_or_default();

            tracing::debug!(?actions, "code actions found");

            let selected = $select.0;
            let selected: &str = &selected.to_string();

            let Some(action) = actions.into_iter().find_map(|a| match a {
                tower_lsp::lsp_types::CodeActionOrCommand::CodeAction(ca)
                    if ca.title == selected =>
                {
                    Some(ca)
                }
                _ => None,
            }) else {
                return Err(
                    format!("failed to find the selected code action '{}'.", selected).into(),
                );
            };
            let Some(edit) = action.edit else {
                return Err("selected code action has no edit".into());
            };
            let mut new_text = toml_text.clone();
            if let Some(doc_changes) = edit.document_changes {
                if let tower_lsp::lsp_types::DocumentChanges::Edits(edits) = doc_changes {
                    for text_edit in edits.into_iter().flat_map(|e| e.edits) {
                        if let tower_lsp::lsp_types::OneOf::Left(edit) = text_edit {
                            let mut lines: Vec<_> =
                                new_text.lines().map(|l| l.to_string()).collect();
                            let start = edit.range.start;
                            let end = edit.range.end;
                            if start.line == end.line {
                                let line = &mut lines[start.line as usize];
                                *line = format!(
                                    "{}{}{}",
                                    &line[..start.character as usize],
                                    edit.new_text,
                                    &line[end.character as usize..]
                                );
                            } else {
                                // 複数行編集は簡易対応
                                lines.splice(
                                    start.line as usize..=end.line as usize,
                                    vec![format!(
                                        "{}{}{}",
                                        &lines[start.line as usize][..start.character as usize],
                                        edit.new_text,
                                        &lines[end.line as usize][end.character as usize..]
                                    )],
                                );
                            }
                            new_text = lines.join("\n");
                        }
                    }
                }
            }
            pretty_assertions::assert_eq!(new_text, textwrap::dedent($expected).trim());
            Ok(())
        }
    };
}

struct Select<T>(T);

mod refactor_rewrite {
    use super::*;
    test_code_action_refactor_rewrite! {
        #[tokio::test]
        async fn dotted_keys_to_inline_table(
            r#"
            foo.bar█ = 1
            "#,
            Select(CodeActionRefactorRewriteName::DottedKeysToInlineTable),
            Option::<std::path::PathBuf>::None,
        ) -> Ok(
            r#"
            foo = { bar = 1 }
            "#
        );
    }
    test_code_action_refactor_rewrite! {
        #[tokio::test]
        async fn inline_table_to_dotted_keys(
            r#"
            foo = { bar = 1█ }
            "#,
            Select(CodeActionRefactorRewriteName::InlineTableToDottedKeys),
            Option::<std::path::PathBuf>::None,
        ) -> Ok(
            r#"
            foo.bar = 1
            "#
        );
    }
}
