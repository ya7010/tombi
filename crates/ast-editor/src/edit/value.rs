use futures::FutureExt;

impl crate::Edit for ast::Value {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            match self {
                ast::Value::Array(array) => {
                    array
                        .edit(
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                }
                ast::Value::InlineTable(inline_table) => {
                    inline_table
                        .edit(
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                }
                _ => Vec::with_capacity(0),
            }
        }
        .boxed()
    }
}
