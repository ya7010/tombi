use futures::FutureExt;

impl crate::Edit for ast::Value {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        current_schema: Option<&'a schema_store::CurrentSchema<'a>>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            match self {
                ast::Value::Array(array) => {
                    array.edit(accessors, current_schema, schema_context).await
                }
                ast::Value::InlineTable(inline_table) => {
                    inline_table
                        .edit(accessors, current_schema, schema_context)
                        .await
                }
                _ => Vec::with_capacity(0),
            }
        }
        .boxed()
    }
}
