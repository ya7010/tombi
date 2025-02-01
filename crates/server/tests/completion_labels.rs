use schema_store::DEFAULT_CATALOG_URL;

use test_lib::{today_local_date, today_local_date_time, today_local_time, today_offset_date_time};

mod completion_labels {
    use super::*;

    mod tombi_schema {
        use super::*;
        use test_lib::tombi_schema_path;

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_empty(
                "█",
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
                "toml-version",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_used_toml_version(
                r#"
                toml-version = "v1.0.0"
                █
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
                // "toml-version",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_empty_bracket(
                "[█]",
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_empty_bracket2(
                r#"
                toml-version = "v1.0.0"

                [█]
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_empty_bracket3(
                r#"
                toml-version = "v1.0.0"

                [█]

                [format]
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_empty_double_bracket(
                "[[█]]",
                tombi_schema_path(),
            ) -> Ok([
                "format",
                "lint",
                "schema",
                "schemas",
                "server",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema(
                r#"
                [schema.█]
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "catalog",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema_after_bracket(
                "[schema]█",
                tombi_schema_path(),
            ) -> Ok([]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema_catalog(
                "[schema.catalog.█]",
                tombi_schema_path(),
            ) -> Ok([]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema_catalog_path(
                r#"
                [schema.catalog]
                path =█
                "#,
                tombi_schema_path(),
            ) -> Ok([
                format!("\"{}\"", DEFAULT_CATALOG_URL),
                "\"\"",
                "''",
                "[]",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema_catalog_path2(
                r#"
                [schema.catalog]
                path = █
                "#,
                tombi_schema_path(),
            ) -> Ok([
                format!("\"{}\"", DEFAULT_CATALOG_URL),
                "\"\"",
                "''",
                "[]",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_schema_catalog_path_inline(
                r#"
                schema.catalog.path =█
                "#,
                tombi_schema_path(),
            ) -> Ok([
                format!("\"{}\"", DEFAULT_CATALOG_URL),
                "\"\"",
                "''",
                "[]",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server2(
                r#"
                [server]
                █
                completion.enabled = true
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "completion",
                "diagnostics",
                "formatting",
                "hover",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server3(
                r#"
                [server]
                formatting.enabled = true
                █
                completion.enabled = true
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "completion",
                "diagnostics",
                "formatting",
                "hover",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server_completion(
                r#"
                [server]
                completion.enabled = █
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "true",
                "false",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server_comp(
                r#"
                [server]
                comp█
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "completion",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server_comp2(
                r#"
                [server.comp█]
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "completion",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn tombi_server_comp3(
                r#"
                [server]
                comp█

                [schema]
                "#,
                tombi_schema_path(),
            ) -> Ok([
                "completion",
            ]);
        }
    }

    mod pyproject_schema {
        use super::*;
        use test_lib::pyproject_schema_path;

        test_completion_labels! {
            #[tokio::test]
            async fn pyproject_empty(
                "█",
                pyproject_schema_path(),
            ) -> Ok([
                "build-system",
                "dependency-groups",
                "project",
                "tool",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn pyproject_project(
                r#"
                [project]
                █
                "#,
                pyproject_schema_path(),
            ) -> Ok([
                "authors",
                "classifiers",
                "dependencies",
                "description",
                "dynamic",
                "keywords",
                "license",
                "license-files",
                "maintainers",
                "name",
                "readme",
                "requires-python",
                "version",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn pyproject_tool_third_party_field(
                r#"
                [tool.third_party]
                field█
                "#,
                pyproject_schema_path(),
            ) -> Ok([
                "{}",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn pyproject_tool_third_party_field_equal(
                r#"
                [tool.third_party]
                field=█
                "#,
                pyproject_schema_path(),
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }
    }

    mod cargo_schema {
        use super::*;
        use test_lib::cargo_schema_path;

        test_completion_labels! {
            #[tokio::test]
            async fn cargo_empty(
                "█",
                cargo_schema_path(),
            ) -> Ok([
                "badges",
                "bench",
                "bin",
                "build-dependencies",
                "build_dependencies",
                "cargo-features",
                "dependencies",
                "dev-dependencies",
                "dev_dependencies",
                "example",
                "features",
                "lib",
                "lints",
                "package",
                "patch",
                "profile",
                "project",
                "replace",
                "target",
                "test",
                "workspace",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn cargo_dependencies_serde_work(
                r#"
                [dependencies]
                serde = { work█ }
                "#,
                cargo_schema_path(),
            ) -> Ok([
                "workspace",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn cargo_dependencies_serde_workspace_dot(
                r#"
                [dependencies]
                serde = { workspace.█ }
                "#,
                cargo_schema_path(),
            ) -> Ok([
                "true",
                "false",
            ]);
        }
    }

    mod without_schema {
        use super::*;

        test_completion_labels! {
            #[tokio::test]
            async fn key_dot(
                "key.█"
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn key_equal(
                "key=█"
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn keys_dot(
                "key1.key2.█"
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn keys_equal(
                "key1.key2=█"
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }

        test_completion_labels! {
            #[tokio::test]
            async fn keys_equal_array(
                "key1= [█]"
            ) -> Ok([
                "\"\"",
                "''",
                today_local_time(),
                today_local_date(),
                today_local_date_time(),
                today_offset_date_time(),
                "3.14",
                "42",
                "[]",
                "true",
                "false",
            ]);
        }
    }
}

#[macro_export]
macro_rules! test_completion_labels {
    (
        #[tokio::test]
        async fn $name:ident(
            $source:expr,
            $schema_file_path:expr$(,)?
        ) -> Ok([$($label:expr),*$(,)?]);
    ) => {
        test_completion_labels! {
            #[tokio::test]
            async fn _$name(
                $source,
                Some($schema_file_path),
            ) -> Ok([$($label),*]);
        }
    };

    (
        #[tokio::test]
        async fn $name:ident(
            $source:expr$(,)?
        ) -> Ok([$($label:expr),*$(,)?]);
    ) => {
        test_completion_labels! {
            #[tokio::test]
            async fn _$name(
                $source,
                Option::<std::path::PathBuf>::None,
            ) -> Ok([$($label),*]);
        }
    };

    (
        #[tokio::test]
        async fn _$name:ident(
            $source:expr,
            $schema_file_path:expr$(,)?
        ) -> Ok([$($label:expr),*$(,)?]);
    ) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use itertools::Itertools;
            use server::Backend;
            use schema_store::JsonCatalogSchema;
            use std::io::Write;
            use tower_lsp::{
                lsp_types::{
                    DidOpenTextDocumentParams, PartialResultParams, TextDocumentIdentifier,
                    TextDocumentItem, Url, WorkDoneProgressParams, CompletionItem,
                    CompletionParams, TextDocumentPositionParams
                },
                LspService,
            };
            use server::handler::handle_did_open;

            if let Ok(level) = std::env::var("RUST_LOG") {
                    let _ = tracing_subscriber::fmt()
                        .with_env_filter(level)
                        .pretty()
                        .try_init();
            }

            let (service, _) = LspService::new(|client| Backend::new(client));

            let backend = service.inner();

            if let Some(schema_file_path) = $schema_file_path.as_ref() {
                let schema_url = Url::from_file_path(schema_file_path).expect(
                    format!(
                        "failed to convert schema path to URL: {}",
                        schema_file_path.display()
                    )
                    .as_str(),
                );
                backend
                    .schema_store
                    .add_catalog(JsonCatalogSchema {
                        name: "test_schema".to_string(),
                        description: "schema for testing".to_string(),
                        file_match: vec!["*.toml".to_string()],
                        url: schema_url.clone(),
                    })
                    .await;
            }

            let Ok(temp_file) = tempfile::NamedTempFile::with_suffix_in(
                ".toml",
                std::env::current_dir().expect("failed to get current directory"),
            ) else {
                return Err("failed to create a temporary file for the test data".into());
            };

            let mut toml_text = textwrap::dedent($source).trim().to_string();

            let Some(index) = toml_text
                .as_str()
                .find("█")
                else {
                    return Err("failed to find completion position marker (█) in the test data".into());
                };

            toml_text.remove(index);
            if temp_file.as_file().write_all(toml_text.as_bytes()).is_err() {
                return  Err("failed to write test data to the temporary file, which is used as a text document".into());
            };

            let Ok(toml_file_url) = Url::from_file_path(temp_file.path()) else {
                return Err("failed to convert temporary file path to URL".into());
            };

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

            let Ok(Some(completions)) = server::handler::handle_completion(
                &backend,
                CompletionParams {
                    text_document_position: TextDocumentPositionParams {
                        text_document: TextDocumentIdentifier { uri: toml_file_url },
                        position: (text::Position::default()
                            + text::RelativePosition::of(&toml_text[..index]))
                        .into(),
                    },
                    work_done_progress_params: WorkDoneProgressParams::default(),
                    partial_result_params: PartialResultParams {
                        partial_result_token: None,
                    },
                    context: None,
                },
            )
            .await else {
                return Err("failed to handle completion".into());
            };

            let labels = completions
                .into_iter()
                .map(|content| Into::<CompletionItem>::into(content))
                .sorted_by(|a, b|
                    a.sort_text.as_ref().unwrap_or(&a.label).cmp(&b.sort_text.as_ref().unwrap_or(&b.label))
                )
                .map(|item| item.label)
                .collect::<Vec<_>>();

            pretty_assertions::assert_eq!(
                labels,
                vec![$($label.to_string()),*] as Vec<String>,
            );

            Ok(())
        }
    };
}
