use std::borrow::Cow;

use ast::AstNode;
use futures::FutureExt;
use schema_store::{AnyOfSchema, CurrentSchema, OneOfSchema, ValueSchema};

impl crate::Edit for ast::Table {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("schema_url: {:?}", schema_url.map(|url| url.to_string()));
        tracing::trace!("value_schema: {:?}", value_schema);

        async move {
            let mut changes = vec![];
            for key_value in self.key_values() {
                changes.extend(
                    key_value
                        .edit(
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await,
                );
            }

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                changes.extend(table_keys_order())
            }

            changes
        }
        .boxed()
    }
}
