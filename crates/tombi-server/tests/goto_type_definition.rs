use tombi_test_lib::{cargo_schema_path, pyproject_schema_path, tombi_schema_path};
mod goto_type_definition_tests {
    use super::*;

    mod tombi_schema {
        use super::*;

        test_goto_type_definition!(
            #[tokio::test]
            async fn tombi_toml_version(
                r#"
                toml-version = "█v1.0.0"
                "#,
                Some(tombi_schema_path()),
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn tombi_schema_catalog_path(
                r#"
                [schema.catalog]
                path = "█https://www.schemastore.org/api/json/catalog.json"
                "#,
                Some(tombi_schema_path())
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn tombi_schemas(
                r#"
                [[schemas█]]
                "#,
                Some(tombi_schema_path()),
            ) -> Ok(true);
        );
    }

    mod cargo_schema {
        use super::*;

        test_goto_type_definition!(
            #[tokio::test]
            async fn cargo_package_name(
                r#"
                [package]
                name█ = "tombi"
                "#,
                Some(cargo_schema_path()),
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn cargo_package_readme(
                r#"
                [package]
                readme = "█README.md"
                "#,
                Some(cargo_schema_path()),
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn cargo_dependencies_key(
                r#"
                [dependencies]
                serde█ = { workspace = true }
                "#,
                Some(cargo_schema_path()),
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn cargo_profile_release_strip_debuginfo(
                r#"
                [profile.release]
                strip = "debuginfo█"
                "#,
                Some(cargo_schema_path()),
            ) -> Ok(true);
        );
    }

    mod pyproject_schema {
        use super::*;

        test_goto_type_definition!(
            #[tokio::test]
            async fn pyproject_project_readme(
                r#"
                [project]
                readme = "█1.0.0"
                "#,
                Some(pyproject_schema_path()),
            ) -> Ok(true);
        );

        test_goto_type_definition!(
            #[tokio::test]
            async fn pyproject_dependency_groups(
                r#"
                [dependency-groups]
                dev = [
                    "█pytest>=8.3.3",
                ]
                "#,
                Some(pyproject_schema_path()),
            ) -> Ok(true);
        );
    }

    #[macro_export]
    macro_rules! test_goto_type_definition {
        (#[tokio::test] async fn $name:ident(
            $source:expr,
            $schema_file_path:expr$(,)?
        ) -> Ok($expected:expr);) => {
            #[tokio::test]
            async fn $name() -> Result<(), Box<dyn std::error::Error>> {
                use std::io::Write;
                use tombi_server::handler::{handle_did_open, handle_goto_type_definition};
                use tombi_server::Backend;
                use tower_lsp::{
                    lsp_types::{
                        DidOpenTextDocumentParams, PartialResultParams, TextDocumentIdentifier,
                        TextDocumentItem, TextDocumentPositionParams, Url, WorkDoneProgressParams,
                    },
                    LspService,
                };

                tombi_test_lib::init_tracing();

                let (service, _) = LspService::new(|client| {
                    Backend::new(client, &tombi_server::backend::Options::default())
                });

                let backend = service.inner();

                // Load schema file
                if let Some(schema_file_path) = $schema_file_path {
                    let schema_file_url =
                        tombi_schema_store::SchemaUrl::from_file_path(&schema_file_path).expect(
                            format!(
                                "failed to convert schema path to URL: {}",
                                schema_file_path.display()
                            )
                            .as_str(),
                        );
                    backend
                        .schema_store
                        .load_schemas(
                            &[tombi_config::Schema::Root(tombi_config::RootSchema {
                                toml_version: None,
                                path: schema_file_url.to_string(),
                                include: vec!["*.toml".to_string()],
                            })],
                            None,
                        )
                        .await;
                }

                let Ok(temp_file) = tempfile::NamedTempFile::with_suffix_in(
                    ".toml",
                    std::env::current_dir().expect("failed to get current directory"),
                ) else {
                    return Err("failed to create a temporary file for the test data".into());
                };

                let mut toml_text = textwrap::dedent($source).trim().to_string();

                let Some(index) = toml_text.as_str().find("█") else {
                    return Err("failed to find position marker (█) in the test data".into());
                };

                toml_text.remove(index);
                if temp_file.as_file().write_all(toml_text.as_bytes()).is_err() {
                    return Err("failed to write to temporary file".into());
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

                let params = tower_lsp::lsp_types::request::GotoTypeDefinitionParams {
                    text_document_position_params: TextDocumentPositionParams {
                        text_document: TextDocumentIdentifier { uri: toml_file_url },
                        position: (tombi_text::Position::default()
                            + tombi_text::RelativePosition::of(&toml_text[..index]))
                        .into(),
                    },
                    work_done_progress_params: WorkDoneProgressParams::default(),
                    partial_result_params: PartialResultParams::default(),
                };

                let Ok(result) = handle_goto_type_definition(&backend, params).await else {
                    return Err("failed to handle goto_type_definition".into());
                };

                tracing::debug!("goto_type_definition result: {:#?}", result);

                // Check if type definition link is returned when schema is specified
                if $expected {
                    assert!(result.is_some(), "No type definition link was returned");
                } else {
                    assert!(
                        result.is_none(),
                        "Type definition link was returned but not expected"
                    );
                }

                Ok(())
            }
        };
    }
}
