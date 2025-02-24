mod array_of_tables;
mod key_value;
mod root;
mod table;

use std::borrow::Cow;

use futures::FutureExt;
use schema_store::{CurrentSchema, ValueSchema};

pub trait Edit {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::Accessor],
        schema_url: Option<&'a schema_store::SchemaUrl>,
        value_schema: Option<&'a schema_store::ValueSchema>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>>;
}

fn search_table_schema<'a: 'b, 'b>(
    accessors: &'a [schema_store::Accessor],
    schema_url: &'a schema_store::SchemaUrl,
    value_schema: &'a schema_store::ValueSchema,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a schema_store::SchemaContext<'a>,
) -> futures::future::BoxFuture<'b, Option<&'a schema_store::TableSchema>> {
    async move {
        match value_schema {
            ValueSchema::Table(table_schema) => return Some(table_schema),
            ValueSchema::OneOf(one_of_schema) => {
                for schema in one_of_schema.schemas.write().await.iter_mut() {
                    if let Ok(CurrentSchema {
                        value_schema,
                        schema_url: Some(schema_url),
                        definitions,
                    }) = schema
                        .resolve(
                            Some(Cow::Borrowed(schema_url)),
                            definitions,
                            schema_context.store,
                        )
                        .await
                    {
                        if let Some(_table_schema) = search_table_schema(
                            accessors,
                            &schema_url,
                            &value_schema,
                            &definitions,
                            schema_context,
                        )
                        .await
                        {}
                    }
                }
            }
            _ => {}
        }
        None
    }
    .boxed()
}
