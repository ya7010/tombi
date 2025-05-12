#[macro_export]
macro_rules! test_folding_range {
    (#[tokio::test] async fn $name:ident(
        $source:expr,
    ) -> [$($expected:expr),* $(,)?];) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use tombi_test_lib::tombi_schema_path;
            use tombi_lsp::handler::{handle_did_open, handle_folding_range};
            use tombi_lsp::Backend;
            use tower_lsp::{
                lsp_types::{
                    DidOpenTextDocumentParams, FoldingRangeParams, PartialResultParams, TextDocumentIdentifier,
                    TextDocumentItem, Url, WorkDoneProgressParams,
                },
                LspService,
            };

            let (service, _) = LspService::new(|client| {
                Backend::new(client, &tombi_lsp::backend::Options::default())
            });
            let backend = service.inner();

            let toml_file_url = Url::from_file_path(tombi_schema_path()).expect("failed to convert file path to URL");
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

            let params = FoldingRangeParams {
                text_document: TextDocumentIdentifier { uri: toml_file_url },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            };

            let result = handle_folding_range(backend, params).await?.unwrap();

            let expected: Vec<std::ops::Range<u32>> = vec![$($expected),*];

            let actual: Vec<std::ops::Range<u32>> = result
                .into_iter()
                .map(|r| r.start_line..r.end_line)
                .collect();

            pretty_assertions::assert_eq!(actual, expected);

            Ok(())
        }
    };
}

mod folding_range_tests {
    use super::*;

    test_folding_range!(
        #[tokio::test]
        async fn simple_table_comment(
            r#"
            # comment1
            # comment2
            [table]
            key = "value"
            "#,
        ) -> [
            0..1,
            2..3
        ];
    );

    test_folding_range!(
        #[tokio::test]
        async fn multiple_tables(
            r#"
            # First table
            [table1]
            key1 = "value1"
            key2 = "value2"

            # Second table
            [table2]
            key3 = "value3"
            "#,
        ) -> [
            0..0,
            1..3,
            5..5,
            6..7
        ];
    );

    test_folding_range!(
        #[tokio::test]
        async fn array_of_tables(
            r#"
            [[array]]
            # Array item 1
            key1 = "value1"

            [[array]]
            # Array item 2
            key2 = "value2"
            "#,
        ) -> [
            0..2,
            1..1,
            4..6,
            5..5
        ];
    );
}
