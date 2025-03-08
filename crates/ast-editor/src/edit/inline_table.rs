use futures::FutureExt;
use itertools::Itertools;
use schema_store::ValueSchema;

use crate::rule::{inline_table_comma_tailing_comment, inline_table_keys_order};

impl crate::Edit for ast::InlineTable {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            let mut changes = vec![];

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                if let ValueSchema::Table(table_schema) = value_schema {
                    changes.extend(
                        inline_table_keys_order(
                            self.key_values_with_comma().into_iter().collect_vec(),
                            table_schema,
                            schema_context,
                        )
                        .await,
                    );

                    for key_value in self.key_values() {
                        changes.extend(
                            key_value
                                .edit(
                                    &accessors,
                                    Some(&value_schema),
                                    Some(schema_url),
                                    Some(definitions),
                                    schema_context,
                                )
                                .await,
                        );
                    }

                    return changes;
                }
            }
            for (key_value, comma) in self.key_values_with_comma() {
                changes.extend(inline_table_comma_tailing_comment(
                    &key_value,
                    comma.as_ref(),
                    schema_context,
                ));
                changes.extend(
                    key_value
                        .edit(accessors, None, None, None, schema_context)
                        .await,
                );
            }

            changes
        }
        .boxed()
    }
}
