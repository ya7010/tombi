mod table_keys_order {
    use super::*;

    mod pyproject {
        use super::test_table_keys_order;
        use test_lib::pyproject_schema_path;

        test_table_keys_order! {
            #[tokio::test]
            async fn test_project(
                r#"
                [project]
                version = "0.1.0"
                readme = "README.md"
                description = "A test project"
                name = "test-project"
                requires-python = ">=3.8"
                authors = [
                    {name = "Test Author", email = "test@example.com"}
                ]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "test-project"
                version = "0.1.0"
                description = "A test project"
                readme = "README.md"
                requires-python = ">=3.8"
                authors = [{ name = "Test Author", email = "test@example.com" }]
                "#
            )
        }

        test_table_keys_order! {
            #[tokio::test]
            async fn test_tool_poetry_dependencies(
                r#"
                [project]
                name = "test-project"
                version = "0.1.0"
                description = "A test project"
                authors = [{ name = "test-user" }]
                readme = "README.md"

                [tool.poetry.dependencies]
                python = ">=3.11 <3.13"
                pydantic = "^2.5"
                pandas = "^2.2.0"
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "test-project"
                version = "0.1.0"
                description = "A test project"
                readme = "README.md"
                authors = [{ name = "test-user" }]

                [tool.poetry.dependencies]
                pandas = "^2.2.0"
                pydantic = "^2.5"
                python = ">=3.11 <3.13"
                "#
            )
        }
    }

    #[macro_export]
    macro_rules! test_table_keys_order {
        (
                #[tokio::test]
                async fn $name:ident(
                    $source:expr,
                    $schema_path:expr$(,)?
                ) -> Ok($expected:expr$(,)?)
            ) => {
            #[tokio::test]
            async fn $name() {
                use config::TomlVersion;
                use formatter::{FormatOptions, Formatter};
                use schema_store::SchemaStore;
                use textwrap::dedent;

                if let Ok(level) = std::env::var("RUST_LOG") {
                    let _ = tracing_subscriber::fmt()
                        .with_env_filter(level)
                        .pretty()
                        .try_init();
                }

                // Initialize schema store
                let schema_store = SchemaStore::new(false);

                // Load schemas
                schema_store
                    .load_schemas(
                        &[config::Schema::Root(config::RootSchema {
                            toml_version: None,
                            path: $schema_path.to_string_lossy().to_string(),
                            include: vec!["*.toml".to_string()],
                        })],
                        None,
                    )
                    .await;

                // Initialize formatter
                let format_options = FormatOptions::default();
                let source_path = std::path::PathBuf::from("pyproject.toml");
                let formatter = Formatter::try_new(
                    TomlVersion::default(),
                    formatter::formatter::definitions::FormatDefinitions::default(),
                    &format_options,
                    Some(itertools::Either::Right(source_path.as_path())),
                    &schema_store,
                )
                .await
                .unwrap();

                // Test that keys are reordered according to schema order
                let source = dedent($source).trim().to_string();
                let expected = dedent($expected).trim().to_string() + "\n";

                let formatted = formatter.format(&source).await.unwrap();
                pretty_assertions::assert_eq!(formatted, expected);
            }
        };
    }
}
