use futures::FutureExt;

impl crate::Edit for ast::Root {
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

            for item in self.items() {
                changes.extend(match item {
                    ast::RootItem::Table(table) => {
                        table
                            .edit(
                                &accessors,
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                    }
                    ast::RootItem::ArrayOfTables(array_of_tables) => {
                        array_of_tables
                            .edit(
                                accessors,
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                    }
                    ast::RootItem::KeyValue(key_value) => {
                        key_value
                            .edit(
                                accessors,
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                    }
                });
            }

            changes
        }
        .boxed()
    }
}
