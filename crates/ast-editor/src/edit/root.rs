use futures::FutureExt;

impl crate::Edit for ast::Root {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::Accessor],
        schema_url: Option<&'a schema_store::SchemaUrl>,
        value_schema: Option<&'a schema_store::ValueSchema>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, ()> {
        async move {
            for item in self.items() {
                match item {
                    ast::RootItem::Table(table) => {
                        table
                            .edit(
                                accessors,
                                schema_url,
                                value_schema,
                                definitions,
                                schema_context,
                            )
                            .await;
                    }
                    ast::RootItem::ArrayOfTables(array_of_tables) => {
                        array_of_tables
                            .edit(
                                accessors,
                                schema_url,
                                value_schema,
                                definitions,
                                schema_context,
                            )
                            .await;
                    }
                    ast::RootItem::KeyValue(key_value) => {
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
                }
            }
        }
        .boxed()
    }
}
