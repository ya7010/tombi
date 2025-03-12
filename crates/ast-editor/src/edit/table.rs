use document_tree::TryIntoDocumentTree;
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

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                if let Ok(table) = self
                    .clone()
                    .try_into_document_tree(schema_context.toml_version)
                {
                    let mut value = &document_tree::Value::Table(table);
                    if let Some(value_schema) = get_schema(
                        value,
                        &header_accessors,
                        value_schema,
                        schema_url,
                        definitions,
                        schema_context,
                    )
                    .await
                    {
                        for header_accessor in &header_accessors {
                            match (value, header_accessor) {
                                (document_tree::Value::Table(table), SchemaAccessor::Key(key)) => {
                                    value = table.get(key).unwrap()
                                }
                                (document_tree::Value::Array(array), SchemaAccessor::Index) => {
                                    value = array.get(0).unwrap()
                                }
                                _ => {}
                            }
                        }

                        for key_value in self.key_values() {
                            changes.extend(
                                key_value
                                    .edit(
                                        &header_accessors,
                                        Some(&value_schema),
                                        Some(schema_url),
                                        Some(definitions),
                                        schema_context,
                                    )
                                    .await,
                            );
                        }

                        changes.extend(
                            table_keys_order(
                                value,
                                self.key_values().collect_vec(),
                                &value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await,
                        );

                        return changes;
                    }
                };
            }
            for key_value in self.key_values() {
                changes.extend(
                    key_value
                        .edit(&header_accessors, None, None, None, schema_context)
                        .await,
                );
            }

            changes
        }
        .boxed()
    }
}
