use std::borrow::Cow;

use futures::FutureExt;
use itertools::Itertools;
use schema_store::{CurrentSchema, ValueSchema};

use crate::rule::array_values_order;

impl crate::Edit for ast::Array {
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
            tracing::error!("schema_url: {:?}", schema_url);
            tracing::error!("value_schema: {:?}", value_schema);

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                if let ValueSchema::Array(array_schema) = &value_schema {
                    if let Some(item_schema) = &array_schema.items {
                        if let Ok(Some(CurrentSchema {
                            schema_url,
                            value_schema,
                            definitions,
                        })) = item_schema
                            .write()
                            .await
                            .resolve(
                                Cow::Borrowed(schema_url),
                                Cow::Borrowed(definitions),
                                schema_context.store,
                            )
                            .await
                        {
                            for value in self.values() {
                                changes.extend(
                                    value
                                        .edit(
                                            accessors,
                                            Some(value_schema),
                                            Some(&schema_url),
                                            Some(&definitions),
                                            schema_context,
                                        )
                                        .await,
                                );
                            }
                        }
                    }
                }
                changes.extend(
                    array_values_order(
                        self.values().into_iter().collect_vec(),
                        &value_schema,
                        schema_context,
                    )
                    .await,
                );
            } else {
                for value in self.values() {
                    changes.extend(
                        value
                            .edit(accessors, None, None, None, schema_context)
                            .await,
                    );
                }
            }

            changes
        }
        .boxed()
    }
}
