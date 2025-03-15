use document_tree::IntoDocumentTreeAndErrors;
use futures::FutureExt;
use itertools::Itertools;
use schema_store::{GetHeaderSchemarAccessors, SchemaAccessor};

use crate::{edit::get_schema, rule::table_keys_order};

impl crate::Edit for ast::Table {
    fn edit<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        tracing::trace!("schema_url: {:?}", schema_url.map(|url| url.to_string()));
        tracing::trace!("value_schema: {:?}", value_schema);

        async move {
            let mut changes = vec![];
            let Some(header_accessors) =
                self.get_header_schema_accessor(schema_context.toml_version)
            else {
                return changes;
            };

            let mut value = &document_tree::Value::Table(
                self.clone()
                    .into_document_tree_and_errors(schema_context.toml_version)
                    .tree,
            );

            let value_schema = if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                get_schema(
                    value,
                    &header_accessors,
                    value_schema,
                    schema_url,
                    definitions,
                    schema_context,
                )
                .await
            } else {
                None
            };

            for header_accessor in &header_accessors {
                match (value, header_accessor) {
                    (document_tree::Value::Table(table), SchemaAccessor::Key(key)) => {
                        let Some(v) = table.get(key) else {
                            return changes;
                        };
                        value = v;
                    }
                    (document_tree::Value::Array(array), SchemaAccessor::Index) => {
                        let Some(v) = array.get(0) else {
                            return changes;
                        };
                        value = v;
                    }
                    _ => {}
                }
            }

            for key_value in self.key_values() {
                changes.extend(
                    key_value
                        .edit(
                            &header_accessors,
                            value_schema.as_ref(),
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await,
                );
            }

            changes.extend(
                table_keys_order(
                    value,
                    self.key_values().collect_vec(),
                    value_schema.as_ref(),
                    schema_url,
                    definitions,
                    schema_context,
                )
                .await,
            );

            changes
        }
        .boxed()
    }
}
