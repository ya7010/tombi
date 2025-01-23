use crate::{
    backend,
    hover::{get_hover_content, HoverContent},
    toml,
};
use ast::{algo::ancestors_at_position, AstNode};
use document_tree::TryIntoDocumentTree;
use itertools::Itertools;
use tower_lsp::lsp_types::{HoverParams, TextDocumentPositionParams};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_hover(
    backend: &backend::Backend,
    HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    }: HoverParams,
) -> Result<Option<HoverContent>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_hover");

    let source = toml::try_load(&text_document.uri)?;
    let position = position.into();
    let toml_version = backend.toml_version().await.unwrap_or_default();

    let document_schema = backend
        .schema_store
        .try_get_schema_from_url(&text_document.uri)
        .await
        .ok()
        .flatten();

    let Some(root) = ast::Root::cast(parser::parse(&source, toml_version).into_syntax_node())
    else {
        return Ok(None);
    };

    let Some((keys, range)) = get_hover_range(&root, position, toml_version) else {
        return Ok(None);
    };

    if keys.is_empty() {
        return Ok(None);
    }

    let Ok(root) = root.try_into_document_tree(toml_version) else {
        return Ok(None);
    };

    return Ok(
        get_hover_content(&root, toml_version, position, &keys, document_schema).map(
            |mut content| {
                content.range = range;
                content
            },
        ),
    );
}

fn get_hover_range(
    root: &ast::Root,
    position: text::Position,
    toml_version: config::TomlVersion,
) -> Option<(Vec<document_tree::Key>, Option<text::Range>)> {
    let mut keys_vec = vec![];
    let mut hover_range = None;

    for node in ancestors_at_position(root.syntax(), position) {
        if let Some(array) = ast::Array::cast(node.to_owned()) {
            for (value, comma) in array.values_with_comma() {
                if hover_range.is_none() {
                    let mut range = value.range();
                    if let Some(comma) = comma {
                        range += comma.range()
                    };
                    if range.contains(position) {
                        hover_range = Some(range);
                    }
                }
            }
        };

        let keys = if let Some(kv) = ast::KeyValue::cast(node.to_owned()) {
            if hover_range.is_none() {
                if let Some(inline_table) = ast::InlineTable::cast(node.parent().unwrap()) {
                    for (key_value, comma) in inline_table.key_values_with_comma() {
                        if hover_range.is_none() {
                            let mut range = key_value.range();
                            if let Some(comma) = comma {
                                range += comma.range()
                            };
                            if range.contains(position) {
                                hover_range = Some(range);
                                break;
                            }
                        }
                    }
                } else {
                    hover_range = Some(kv.range());
                }
            }
            kv.keys().unwrap()
        } else if let Some(table) = ast::Table::cast(node.to_owned()) {
            table.header().unwrap()
        } else if let Some(array_of_tables) = ast::ArrayOfTables::cast(node.to_owned()) {
            array_of_tables.header().unwrap()
        } else {
            continue;
        };

        let keys = if keys.range().contains(position) {
            let mut new_keys = Vec::with_capacity(keys.keys().count());
            for key in keys
                .keys()
                .take_while(|key| key.token().unwrap().range().start() <= position)
            {
                match key.try_into_document_tree(toml_version) {
                    Ok(Some(key)) => new_keys.push(key),
                    _ => return None,
                }
            }
            new_keys
        } else {
            let mut new_keys = Vec::with_capacity(keys.keys().count());
            for key in keys.keys() {
                match key.try_into_document_tree(toml_version) {
                    Ok(Some(key)) => new_keys.push(key),
                    _ => return None,
                }
            }
            new_keys
        };

        if hover_range.is_none() {
            hover_range = keys.iter().map(|key| key.range()).reduce(|k1, k2| k1 + k2);
        }

        keys_vec.push(keys);
    }

    Some((
        keys_vec.into_iter().rev().flatten().collect_vec(),
        hover_range,
    ))
}

#[cfg(test)]
mod test {
    use crate::test::{cargo_schema_path, pyproject_schema_path, tombi_schema_path};

    use super::*;

    #[macro_export]
    macro_rules! test_hover_keys_value {
        (#[tokio::test] async fn $name:ident($schema_file_path:expr, $source:expr) -> Ok({
            "Keys": $keys:expr,
            "Value": $value_type:expr
        });) => {
            #[tokio::test]
            async fn $name() {
                use backend::Backend;
                use std::io::Write;
                use tower_lsp::{
                    lsp_types::{TextDocumentIdentifier, Url, WorkDoneProgressParams},
                    LspService,
                };
                use schema_store::JsonCatalogSchema;

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
                    .add_catalog(
                        JsonCatalogSchema{
                            name: "test_schema".to_string(),
                            description: "schema for testing".to_string(),
                            file_match: vec!["*.toml".to_string()],
                            url: schema_url.clone(),
                        }
                    )
                    .await;

                let temp_file = tempfile::NamedTempFile::with_suffix_in(
                        ".toml",
                        std::env::current_dir().expect("failed to get current directory"),
                    )
                    .expect("failed to create temporary file");

                let mut toml_data = textwrap::dedent($source).trim().to_string();

                let pos = toml_data
                    .as_str()
                    .find("█")
                    .expect("failed to find hover position marker (█) in the test data");

                toml_data.remove(pos);
                temp_file.as_file().write_all(toml_data.as_bytes()).expect(
                    "failed to write test data to the temporary file, which is used as a text document",
                );

                let hover_content = handle_hover(
                    &backend,
                    HoverParams {
                        text_document_position_params: TextDocumentPositionParams {
                            text_document: TextDocumentIdentifier {
                                uri: Url::from_file_path(temp_file.path()).expect(
                                    "failed to convert temporary file path to URL for the text document",
                                ),
                            },
                            position: (text::Position::default()
                                + text::RelativePosition::of(&toml_data[..pos]))
                            .into(),
                        },
                        work_done_progress_params: WorkDoneProgressParams::default(),
                    },
                )
                .await
                .expect("failed to handle hover")
                .expect("failed to get hover content");

                assert!(hover_content.schema_url.is_some(), "The hover target is not defined in the schema.");
                pretty_assertions::assert_eq!(hover_content.accessors.to_string(), $keys);
                pretty_assertions::assert_eq!(hover_content.value_type.to_string(), $value_type);
            }
        }
    }

    test_hover_keys_value!(
        #[tokio::test]
        async fn tombi_toml_version(
            tombi_schema_path(),
            r#"
            toml-version = "█v1.0.0"
            "#
        ) -> Ok({
            "Keys": "toml-version",
            "Value": "String?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn tombi_schema_catalog_path(
            tombi_schema_path(),
            r#"
            [schema.catalog]
            path = "█https://www.schemastore.org/api/json/catalog.json"
            "#
        ) -> Ok({
            "Keys": "schema.catalog.path",
            "Value": "(String | Array)?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        // NOTE: This test is correct. When you hover over the last key of the header of ArrayOfTables,
        //       the Keys in the hover content is `schema[$index]`, not `schemas`.
        //       Therefore, the Value is `Table`.
        async fn tombi_schemas(
            tombi_schema_path(),
            r#"
            [[schemas█]]
            "#
        ) -> Ok({
            "Keys": "schemas[0]",
            "Value": "Table"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn tombi_schemas_path(
            tombi_schema_path(),
            r#"
            [[schemas]]
            path = "█tombi.schema.json"
            "#
        ) -> Ok({
            "Keys": "schemas[0].path",
            "Value": "String"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_package_name(
            cargo_schema_path(),
            r#"
            [package]
            name█ = "tombi"
            "#
        ) -> Ok({
            "Keys": "package.name",
            "Value": "String" // Yes; the value is required.
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_package_readme(
            cargo_schema_path(),
            r#"
            [package]
            readme = "█README.md"
            "#
        ) -> Ok({
            "Keys": "package.readme",
            "Value": "(String | Boolean | Table)?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_dependencies_key(
            cargo_schema_path(),
            r#"
            [dependencies]
            serde█ = { workspace = true }
            "#
        ) -> Ok({
            "Keys": "dependencies.serde",
            "Value": "(String | Table)?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_dependencies_version(
            cargo_schema_path(),
            r#"
            [dependencies]
            serde = "█1.0"
            "#
        ) -> Ok({
            "Keys": "dependencies.serde",
            "Value": "(String | Table)?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_dependencies_workspace(
            cargo_schema_path(),
            r#"
            [dependencies]
            serde = { workspace█ = true }
            "#
        ) -> Ok({
            "Keys": "dependencies.serde.workspace",
            "Value": "Boolean?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_dependencies_features(
            cargo_schema_path(),
            r#"
            [dependencies]
            serde = { version = "^1.0.0", features█ = ["derive"] }
            "#
        ) -> Ok({
            "Keys": "dependencies.serde.features",
            "Value": "Array?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn cargo_dependencies_features_item(
            cargo_schema_path(),
            r#"
            [dependencies]
            serde = { version = "^1.0.0", features = ["derive█"] }
            "#
        ) -> Ok({
            "Keys": "dependencies.serde.features[0]",
            "Value": "String"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn pyprpoject_project_readme(
            pyproject_schema_path(),
            r#"
            [project]
            readme = "█1.0.0"
            "#
        ) -> Ok({
            "Keys": "project.readme",
            "Value": "(String ^ Table)?"
        });
    );

    test_hover_keys_value!(
        #[tokio::test]
        async fn pyprpoject_dependency_groups(
            pyproject_schema_path(),
            r#"
            [dependency-groups]
            dev = [
                "█pytest>=8.3.3",
            ]
            "#
        ) -> Ok({
            "Keys": "dependency-groups.dev[0]",
            "Value": "String ^ Table"
        });
    );
}
