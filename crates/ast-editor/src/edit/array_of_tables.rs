use futures::FutureExt;

impl crate::Edit for ast::ArrayOfTables {
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

            changes
        }
        .boxed()
    }
}
