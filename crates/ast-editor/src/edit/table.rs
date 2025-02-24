use ast::AstNode;
use futures::FutureExt;
use schema_store::Accessor;

use super::search_table_schema;

impl crate::Edit for ast::Table {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::Accessor],
        schema_url: Option<&'a schema_store::SchemaUrl>,
        value_schema: Option<&'a schema_store::ValueSchema>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            let mut changes = vec![];
            let mut accessor = vec![];
            for key in self.header().unwrap().keys() {
                let Ok(key_text) = key.try_to_raw_text(schema_context.toml_version) else {
                    return changes;
                };
                accessor.push(Accessor::Key(key_text));
            }

            for key_value in self.key_values() {
                changes.extend(
                    key_value
                        .edit(
                            accessors,
                            schema_url,
                            value_schema,
                            definitions,
                            schema_context,
                        )
                        .await,
                );
            }

            match (schema_url, value_schema, definitions) {
                (Some(schema_url), Some(value_schema), Some(definitions)) => {
                    let Some(table_schema) = search_table_schema(
                        &accessor,
                        schema_url,
                        value_schema,
                        definitions,
                        schema_context,
                    )
                    .await
                    else {
                        return changes;
                    };
                    changes.extend(crate::rule::table_key_order(self.syntax(), table_schema).await);
                }
                _ => {}
            }

            changes
        }
        .boxed()
    }
}
