use ast::AstNode;
use futures::FutureExt;

impl crate::Edit for ast::Table {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::Accessor],
        schema_url: Option<&'a schema_store::SchemaUrl>,
        value_schema: Option<&'a schema_store::ValueSchema>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, ()> {
        async move {
            for key_value in self.key_values() {
                key_value
                    .edit(
                        accessors,
                        schema_url,
                        value_schema,
                        definitions,
                        schema_context,
                    )
                    .await;
            }
            if let Some(value_schema) = value_schema {
                crate::rule::table_key_order(self.syntax(), value_schema);
            }
        }
        .boxed()
    }
}
