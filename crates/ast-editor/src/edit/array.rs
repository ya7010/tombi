use std::borrow::Cow;

use futures::FutureExt;
use itertools::Itertools;
use schema_store::{CurrentSchema, ValueSchema};

use crate::rule::{array_comma_tailing_comment, array_values_order};

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

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                if let ValueSchema::Array(array_schema) = &value_schema {
                    changes.extend(
                        array_values_order(
                            self.values_with_comma().into_iter().collect_vec(),
                            &array_schema,
                            schema_context,
                        )
                        .await,
                    );

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

                            return changes;
                        }
                    }
                }
            }

            for (value, comma) in self.values_with_comma() {
                changes.extend(array_comma_tailing_comment(
                    &value,
                    comma.as_ref(),
                    schema_context,
                ));
                changes.extend(
                    value
                        .edit(accessors, None, None, None, schema_context)
                        .await,
                );
            }

            changes
        }
        .boxed()
    }
}
