mod error;
mod lint;
mod linter;
mod rule;
mod warning;

pub use config::LintOptions;
use diagnostic::Diagnostic;
pub use error::{Error, ErrorKind};
use lint::Lint;
pub use linter::Linter;
use rule::Rule;
pub use warning::{Warning, WarningKind};

#[cfg(test)]
#[macro_export]
macro_rules! test_lint {
    (#[test]
    fn $name:ident(
        $source:expr,
        $schema_path:expr$(,)?
    ) -> Ok($_:tt);) => {
        #[tokio::test]
        async fn $name() {
            use config::TomlVersion;

            if let Ok(level) = std::env::var("RUST_LOG") {
                let _ = tracing_subscriber::fmt()
                    .with_env_filter(level)
                    .pretty()
                    .try_init();
            }

            // Initialize schema store
                let schema_store = schema_store::SchemaStore::new(schema_store::Options::default());

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

            // Initialize linter with schema if provided
            let source_path = std::path::PathBuf::from("test.toml");
            let mut linter = $crate::Linter::try_new(
                TomlVersion::default(),
                &crate::LintOptions::default(),
                Some(itertools::Either::Right(source_path.as_path())),
                &schema_store,
            ).await.unwrap();

            // Perform linting
            let result = linter.lint($source).await;

            // match result {
            //     Ok(_) => {}, // Success as expected
            //     Err(errors) => {
            //         panic!("Expected success but got errors: {:?}", errors);
            //     }
            // }
        }
    };

    (#[test]
    fn $name:ident(
        $source:expr,
        $schema_path:expr$(,)?
    ) -> Err([$($diagnostic:expr),* $(,)?]);) => {
        #[test]
        fn $name() {
            if let Ok(level) = std::env::var("RUST_LOG") {
                let _ = tracing_subscriber::fmt()
                    .with_env_filter(level)
                    .pretty()
                    .try_init();
            }

            // Initialize linter with schema if provided
            let mut linter = $crate::Linter::new($crate::LintOptions::default());
            $(
                linter.load_schema($schema_path);
            )?

            // Perform linting
            let result = linter.lint($source);

            match result {
                Ok(_) => {
                    panic!("Expected errors but got success");
                }
                Err(errors) => {
                    let expected_diagnostics = vec![$($diagnostic),*];
                    assert_eq!(errors, expected_diagnostics, "Diagnostics do not match");
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_lib::cargo_schema_path;

    test_lint! {
        #[test]
        fn test_workspace_dependencies(
            r#"
        [workspace.dependencies]
        serde.version = "^1.0.0"
        serde.features = ["derive"]
        serde.workspace = true
        "#,
            cargo_schema_path(),
        ) -> Ok(_);
    }

    test_lint! {
        #[test]
        fn test_warning_empty(
            r#"
            "" = 1
            "#,
            cargo_schema_path(),
        ) -> Err([
            Diagnostic::new(Warning::EmptyKey, 1, 1..3),
        ]);
    }
}
