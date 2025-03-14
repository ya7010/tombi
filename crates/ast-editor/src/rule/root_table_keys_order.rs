use ast::AstNode;
use document_tree::{DocumentTreeAndErrors, IntoDocumentTreeAndErrors};
use itertools::Itertools;
use schema_store::{SchemaAccessor, SchemaContext, ValueSchema};
use syntax::{SyntaxElement, SyntaxNode};
use toml_version::TomlVersion;

use crate::rule::table_keys_order::{sorted_accessors, table_keys_order};

pub async fn root_table_keys_order<'a>(
    key_values: Vec<ast::KeyValue>,
    table_or_array_of_tables: Vec<TableOrArrayOfTables>,
    value_schema: &'a ValueSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<crate::Change> {
    if key_values.is_empty() && table_or_array_of_tables.is_empty() {
        return Vec::with_capacity(0);
    }

    let mut changes = table_keys_order(
        &document_tree::Value::Table(
            key_values
                .clone()
                .into_document_tree_and_errors(schema_context.toml_version)
                .tree,
        ),
        key_values,
        value_schema,
        schema_url,
        definitions,
        schema_context,
    )
    .await;

    if table_or_array_of_tables.is_empty() {
        return changes;
    }

    let old = std::ops::RangeInclusive::new(
        SyntaxElement::Node(table_or_array_of_tables.first().unwrap().syntax().clone()),
        SyntaxElement::Node(table_or_array_of_tables.last().unwrap().syntax().clone()),
    );

    let targets = table_or_array_of_tables
        .clone()
        .into_iter()
        .map(|table| {
            (
                table
                    .header()
                    .map(|key| {
                        key.keys()
                            .into_iter()
                            .map(|key| {
                                SchemaAccessor::Key(
                                    key.try_to_raw_text(schema_context.toml_version).unwrap(),
                                )
                            })
                            .collect_vec()
                    })
                    .unwrap_or_default(),
                table,
            )
        })
        .collect_vec();

    let ValueSchema::Table(table_schema) = value_schema else {
        return Vec::with_capacity(0);
    };

    let new = sorted_accessors(
        &document_tree::Value::Table(
            table_or_array_of_tables
                .into_document_tree_and_errors(schema_context.toml_version)
                .tree,
        ),
        &[],
        targets,
        &ValueSchema::Table(table_schema.clone()),
        schema_url,
        definitions,
        schema_context,
    )
    .await
    .into_iter()
    .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
    .collect_vec();

    changes.push(crate::Change::ReplaceRange { old, new });

    changes
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableOrArrayOfTables {
    Table(ast::Table),
    ArrayOfTables(ast::ArrayOfTables),
}

impl TableOrArrayOfTables {
    fn header(&self) -> Option<ast::Keys> {
        match self {
            Self::Table(table) => table.header(),
            Self::ArrayOfTables(array_of_tables) => array_of_tables.header(),
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Table(table) => table.syntax(),
            Self::ArrayOfTables(array_of_tables) => array_of_tables.syntax(),
        }
    }
}

impl IntoDocumentTreeAndErrors<document_tree::Table> for TableOrArrayOfTables {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> DocumentTreeAndErrors<document_tree::Table> {
        match self {
            Self::Table(table) => table.into_document_tree_and_errors(toml_version),
            Self::ArrayOfTables(array_of_tables) => {
                array_of_tables.into_document_tree_and_errors(toml_version)
            }
        }
    }
}
