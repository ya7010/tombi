use tombi_test_lib::project_root_path;
use tower_lsp::lsp_types::{DocumentLink, Position, Range};

mod document_link_tests {
    use super::*;

    mod cargo_schema {
        use super::*;

        test_document_link!(
            #[tokio::test]
            async fn cargo_package_readme(
                r#"
                [package]
                readme = "README.md"
                "#,
                project_root_path().join("Cargo.toml"),
            ) -> Ok(Some(vec![]));
        );

        test_document_link!(
            #[tokio::test]
            async fn cargo_package_readme_without_schema(
                r#"
                #:schema schemas/Cargo.json

                [package]
                readme = "README.md"
                "#,
                project_root_path().join("Cargo.toml"),
            ) -> Ok(Some(vec![
                DocumentLink {
                    range: Range {
                        start: Position { line: 0, character: 9 },
                        end: Position { line: 0, character: 27 },
                    },
                    target: Url::from_file_path(
                        project_root_path().join("schemas/Cargo.json"),
                    ).ok(),
                    tooltip: Some("Open JSON Schema".to_string()),
                    data: None,
                }
            ]));
        );
    }
}

#[macro_export]
macro_rules! test_document_link {
    (#[tokio::test] async fn $name:ident(
        $source:expr,
        $file_path:expr,
    ) -> Ok($expected_links:expr);) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use tombi_lsp::handler::{handle_did_open, handle_document_link};
            use tombi_lsp::Backend;
            use tower_lsp::{
                lsp_types::{
                    DidOpenTextDocumentParams, DocumentLinkParams, PartialResultParams,
                    TextDocumentIdentifier, TextDocumentItem, Url, WorkDoneProgressParams,
                },
                LspService,
            };

            tombi_test_lib::init_tracing();

            let (service, _) = LspService::new(|client| {
                Backend::new(client, &tombi_lsp::backend::Options::default())
            });

            let backend = service.inner();

            let toml_file_url =
                Url::from_file_path($file_path).expect("failed to convert file path to URL");

            let toml_text = textwrap::dedent($source).trim().to_string();

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

            let params = DocumentLinkParams {
                text_document: TextDocumentIdentifier { uri: toml_file_url },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            };

            let result = handle_document_link(&backend, params).await;

            tracing::debug!("document_link result: {:#?}", result);

            pretty_assertions::assert_eq!(result, Ok($expected_links));

            Ok(())
        }
    };
}
