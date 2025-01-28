use tower_lsp::lsp_types::{CompletionParams, TextDocumentPositionParams};

use crate::{
    backend,
    completion::{get_completion_contents, CompletionContent},
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_completion(
    backend: &backend::Backend,
    CompletionParams {
        text_document_position:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    }: CompletionParams,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_completion");

    let config = backend.config().await;

    if !config
        .server
        .and_then(|server| server.completion)
        .and_then(|completion| completion.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.completion.enabled` is false");
        return Ok(None);
    }

    if !config
        .schema
        .and_then(|s| s.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`schema.enabled` is false");
        return Ok(None);
    }

    let Ok(Some(document_schema)) = &backend
        .schema_store
        .try_get_schema_from_url(&text_document.uri)
        .await
    else {
        tracing::debug!("schema not found: {}", text_document.uri);
        return Ok(None);
    };
    let Some(document_source) = backend.get_document_source(&text_document.uri) else {
        return Ok(None);
    };

    // FIXME: Remove whitespaces, because the AST assigns the whitespace to the next section.
    //        In the future, it would be better to move the whitespace in ast_editor.
    let mut position: text::Position = position.into();
    while position.column() != 0 && position.char_at_left(&document_source.source) == Some(' ') {
        position = text::Position::new(position.line(), position.column() - 1);
    }

    let toml_version = backend.toml_version().await.unwrap_or_default();
    let Some(root) = backend.get_incomplete_ast(&text_document.uri, toml_version) else {
        return Ok(None);
    };

    Ok(Some(get_completion_contents(
        root,
        position,
        document_schema,
        toml_version,
    )))
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use schema_store::DEFAULT_CATALOG_URL;

    use crate::test::{cargo_schema_path, pyproject_schema_path, tombi_schema_path};

    use super::*;

    #[macro_export]
    macro_rules! test_completion_labels {
        (
            #[tokio::test]
            async fn $name:ident(
                $source:expr,
                $schema_file_path:expr$(,)?
            ) -> Ok([$($label:expr),*$(,)?]);
        ) => {
            #[tokio::test]
            async fn $name() {
                use backend::Backend;
                use schema_store::JsonCatalogSchema;
                use std::io::Write;
                use tower_lsp::{
                    lsp_types::{
                        DidOpenTextDocumentParams, PartialResultParams, TextDocumentIdentifier,
                        TextDocumentItem, Url, WorkDoneProgressParams, CompletionItem,
                    },
                    LspService,
                };
                use $crate::handler::handle_did_open;

                let (service, _) = LspService::new(|client| Backend::new(client));

                let backend = service.inner();

                let schema_url = Url::from_file_path($schema_file_path).expect(
                    format!(
                        "failed to convert schema path to URL: {}",
                        tombi_schema_path().display()
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

                let temp_file = tempfile::NamedTempFile::with_suffix_in(
                    ".toml",
                    std::env::current_dir().expect("failed to get current directory"),
                )
                .expect("failed to create temporary file");

                let mut toml_text = textwrap::dedent($source).trim().to_string();

                let index = toml_text
                    .as_str()
                    .find("█")
                    .expect("failed to find completion position marker (█) in the test data");

                toml_text.remove(index);
                temp_file.as_file().write_all(toml_text.as_bytes()).expect(
                    "failed to write test data to the temporary file, which is used as a text document",
                );

                let toml_file_url = Url::from_file_path(temp_file.path())
                    .expect("failed to convert temporary file path to URL");

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

                let completions = handle_completion(
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
                .await
                .expect("failed to handle completion")
                .expect("failed to get completion items");

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
            }
        };
    }

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
            r#"
            [schema]█
            "#,
            tombi_schema_path(),
        ) -> Ok([]);
    }

    test_completion_labels! {
        #[tokio::test]
        async fn tombi_schema_catalog(
            r#"
            [schema.catalog.█]
            "#,
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
}
