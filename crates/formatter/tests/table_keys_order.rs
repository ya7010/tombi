mod table_keys_order {
    mod pyproject {
        use std::path::PathBuf;

        use itertools::Either;
        use test_lib::pyproject_schema_path;

        #[tokio::test]
        async fn test_table_keys_order() {
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
                        path: pyproject_schema_path().to_string_lossy().to_string(),
                        include: vec!["*.toml".to_string()],
                    })],
                    None,
                )
                .await;

            // Initialize formatter
            let format_options = FormatOptions::default();
            let source_path = PathBuf::from("pyproject.toml");
            let formatter = Formatter::try_new(
                TomlVersion::default(),
                formatter::formatter::definitions::FormatDefinitions::default(),
                &format_options,
                Some(Either::Right(source_path.as_path())),
                &schema_store,
            )
            .await
            .unwrap();

            // Test that keys in the project section are reordered according to schema order
            let source = dedent(
                r#"
                [project]
                version = "0.1.0"
                description = "A test project"
                name = "test-project"
                readme = "README.md"
                requires-python = ">=3.8"
                authors = [
                    {name = "Test Author", email = "test@example.com"}
                ]
                "#,
            )
            .trim()
            .to_string();

            let expected = dedent(
                r#"
                [project]
                name = "test-project"
                version = "0.1.0"
                description = "A test project"
                readme = "README.md"
                requires-python = ">=3.8"
                authors = [{ name = "Test Author", email = "test@example.com" }]
                "#,
            )
            .trim()
            .to_string()
                + "\n";

            let formatted = formatter.format(&source).await.unwrap();
            pretty_assertions::assert_eq!(formatted, expected);
        }
    }
}
