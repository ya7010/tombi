mod folding_range_tests {
    use super::*;

    test_folding_range!(
        #[tokio::test]
        async fn simple_key_value_comment(
            r#"
            # comment1
            # comment2

            # comment3
            # comment4

            # comment5
            # comment6
            key = "value"

            # comment7
            # comment8
            "#,
        ) -> [0..4, 10..11, 6..7];
    );

    test_folding_range!(
        #[tokio::test]
        async fn simple_table_comment(
            r#"
            # comment1
            # comment2
            [table]
            key = "value"
            "#,
        ) -> [0..1, 2..3];
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
        ) -> [0..0, 1..3, 5..5, 6..7];
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
        ) -> [0..2, 1..1, 4..6, 5..5];
    );

    test_folding_range!(
        #[tokio::test]
        async fn table_with_comments(
            r#"
            # Table header comment1
            # Table header comment2
            [table]
            # Key value begin comment1
            # Key value begin comment2
            key1 = "value1"
            # Key value end comment1
            # Key value end comment2
            "#,
        ) -> [0..1, 2..7, 6..7, 3..4];
    );

    test_folding_range!(
        #[tokio::test]
        async fn array_with_comments(
            r#"
            array = [
              # Array begin comment1
              # Array begin comment2
              1,
              # Value comment1
              # Value comment2
              2,
              # Array end comment1
              # Array end comment2
            ]
            "#,
        ) -> [0..9, 7..8, 1..2, 4..5];
    );

    test_folding_range!(
        #[tokio::test]
        async fn nested_structure_with_comments(
            r#"
            # Root begin comment1
            # Root begin comment2
            [outer]
            # Outer begin comment1
            # Outer begin comment2
            inner = [
                # Inner begin comment1
                # Inner begin comment2
                1,
                # Inner end comment1
                # Inner end comment2
            ]
            # Outer end comment1
            # Outer end comment2
            # Root end comment1
            # Root end comment2
            "#,
        ) -> [0..1, 2..15, 12..15, 3..4, 5..11, 9..10, 6..7];
    );

    test_folding_range!(
        #[tokio::test]
        async fn array_of_tables_with_comments(
            r#"
            # Array of tables begin comment1
            # Array of tables begin comment2
            [[items]]
            # Item begin comment1
            # Item begin comment2
            key1 = 1
            # Item end comment1
            # Item end comment2

            [[items]]
            # Item begin comment1
            # Item begin comment2
            key2 = 2
            # Item end comment1
            # Item end comment2
            # Array of tables end comment1
            # Array of tables end comment2
            "#,
        ) -> [0..1, 2..7, 6..7, 3..4, 9..16, 13..16, 10..11];
    );

    #[macro_export]
    macro_rules! test_folding_range {
        (#[tokio::test] async fn $name:ident($source:expr $(,)?) -> [$($expected:expr),* $(,)?];) => {
            #[tokio::test]
            async fn $name() -> Result<(), Box<dyn std::error::Error>> {
                use tombi_test_lib::tombi_schema_path;
                use tombi_lsp::handler::{handle_did_open, handle_folding_range};
                use tombi_lsp::Backend;
                use tower_lsp::{
                    lsp_types::{
                        DidOpenTextDocumentParams,
                        FoldingRangeParams,
                        PartialResultParams,
                        TextDocumentIdentifier,
                        TextDocumentItem,
                        Url,
                        WorkDoneProgressParams,
                    },
                    LspService,
                };

                let (service, _) = LspService::new(|client| {
                    Backend::new(client, &tombi_lsp::backend::Options::default())
                });
                let backend = service.inner();

                let toml_file_url = Url::from_file_path(tombi_schema_path())
                    .expect("failed to convert file path to URL");
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

                let Ok(Some(result)) = handle_folding_range(backend, params).await else {
                    panic!("failed to get folding range");
                };

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
}
