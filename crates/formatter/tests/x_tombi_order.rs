mod table_keys_order {
    use super::*;

    mod pyproject {
        use super::test_format;
        use test_lib::pyproject_schema_path;

        test_format! {
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

        test_format! {
            #[tokio::test]
            async fn test_project_dependencies_single_line(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = ["tombi-cli>=0.0.0", "maturin>=1.5,<2.0"]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = ["maturin>=1.5,<2.0", "tombi-cli>=0.0.0"]
                "#,
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_project_dependencies_single_line_with_comma(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = ["tombi-cli>=0.0.0", "maturin>=1.5,<2.0",]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = [
                  "maturin>=1.5,<2.0",
                  "tombi-cli>=0.0.0",
                ]
                "#,
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_project_dependencies_multiple_lines(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = [
                  "tombi-linter>=0.0.0",
                  "tombi-formatter>=0.0.0",
                  "maturin>=1.5,<2.0",
                  "tombi-cli>=0.0.0"
                ]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = [
                  "maturin>=1.5,<2.0",
                  "tombi-cli>=0.0.0",
                  "tombi-formatter>=0.0.0",
                  "tombi-linter>=0.0.0",
                ]
                "#,
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_project_dependencies_multiple_lines_with_comment(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = [
                  "tombi-linter>=0.0.0",
                  "tombi-formatter>=0.0.0",
                  # maturin leading comment1
                  # maturin leading comment2
                  "maturin>=1.5,<2.0", # maturin tailing comment
                  # tombi-cli leading comment1
                  # tombi-cli leading comment2
                  "tombi-cli>=0.0.0" # tombi-cli tailing comment
                ]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                description = "Reserved package for tombi"
                requires-python = ">=3.10"
                dependencies = [
                  # maturin leading comment1
                  # maturin leading comment2
                  "maturin>=1.5,<2.0",  # maturin tailing comment
                  # tombi-cli leading comment1
                  # tombi-cli leading comment2
                  "tombi-cli>=0.0.0",  # tombi-cli tailing comment
                  "tombi-formatter>=0.0.0",
                  "tombi-linter>=0.0.0",
                ]
                "#,
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_dependency_groups_multiple_lines_with_comment(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                requires-python = ">=3.10"
                dependencies = []

                [dependency-groups]
                dev = [
                  "pytest>=8.3.3", # pytest tailing comment
                  "ruff>=0.7.4"
                ]
                "#,
                pyproject_schema_path(),
            ) -> Ok(
                r#"
                [project]
                name = "tombi"
                version = "1.0.0"
                requires-python = ">=3.10"
                dependencies = []

                [dependency-groups]
                dev = [
                  "pytest>=8.3.3",  # pytest tailing comment
                  "ruff>=0.7.4",
                ]
                "#,
            )
        }

        test_format! {
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

    mod cargo {
        use test_lib::cargo_schema_path;

        use super::*;

        test_format! {
            #[tokio::test]
            async fn test_cargo_package(
                r#"
                [package]
                name = "toml-version"
                authors.workspace = true
                edition.workspace = true
                license.workspace = true
                repository.workspace = true
                version.workspace = true
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [package]
                name = "toml-version"
                version.workspace = true
                authors.workspace = true
                edition.workspace = true
                repository.workspace = true
                license.workspace = true
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_cargo_package2(
                r#"
                [package]
                name = "toml-version"
                authors = { workspace = true }
                edition = { workspace = true }
                license = { workspace = true }
                repository = { workspace = true }
                version = { workspace = true }
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [package]
                name = "toml-version"
                version = { workspace = true }
                authors = { workspace = true }
                edition = { workspace = true }
                repository = { workspace = true }
                license = { workspace = true }
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_dependencies_and_features(
                r#"
                [features]
                default = ["clap"]
                clap = ["clap/derive"]

                [dependencies]
                serde = { features = ["derive"], version = "^1.0.0" }
                clap = { version = "4.5.0" }
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [dependencies]
                clap = { version = "4.5.0" }
                serde = { version = "^1.0.0", features = ["derive"] }

                [features]
                clap = ["clap/derive"]
                default = ["clap"]
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_cargo_dependencies(
                r#"
                [dependencies]
                serde = { features = ["derive"], version = "^1.0.0" }
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [dependencies]
                serde = { version = "^1.0.0", features = ["derive"] }
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_cargo_dependencies_trailing_comma(
                r#"
                [dependencies]
                serde = { features = ["std", "derive",], version = "^1.0.0" }
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [dependencies]
                serde = { version = "^1.0.0", features = [
                  "derive",
                  "std",
                ] }
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_workspace_dependencies(
                r#"
                [workspace.dependencies]
                serde.version = "^1.0.0"
                serde.features = ["derive"]
                serde.workspace = true
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [workspace.dependencies]
                serde.workspace = true
                serde.version = "^1.0.0"
                serde.features = ["derive"]
                "#
            )
        }

        test_format! {
            #[tokio::test]
            async fn test_workspace_dependencies_complex(
                r#"
                [workspace.dependencies]
                serde.version = "^1.0.0"
                serde.workspace = true
                serde.features = ["derive"]
                anyhow = "1.0.89"
                chrono = { version = "0.4.38", features = ["serde"] }
                reqwest.default-features = false
                reqwest.version = "0.12.9"
                reqwest.features = ["json", "rustls-tls"]
                "#,
                cargo_schema_path(),
            ) -> Ok(
                r#"
                [workspace.dependencies]
                anyhow = "1.0.89"
                chrono = { version = "0.4.38", features = ["serde"] }
                reqwest.version = "0.12.9"
                reqwest.default-features = false
                reqwest.features = ["json", "rustls-tls"]
                serde.workspace = true
                serde.version = "^1.0.0"
                serde.features = ["derive"]
                "#
            )
        }
    }

    mod tombi {
        use super::test_format;
        use test_lib::tombi_schema_path;

        test_format! {
            #[tokio::test]
            async fn test_tombi(
                r#"
                [[schemas]]
                include = ["*.toml"]
                path = "pyproject.toml"
                "#,
                tombi_schema_path(),
            ) -> Ok(
                r#"
                [[schemas]]
                path = "pyproject.toml"
                include = ["*.toml"]
                "#
            )
        }
    }

    #[macro_export]
    macro_rules! test_format {
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
                let schema_store = SchemaStore::new(schema_store::Options::default());

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
