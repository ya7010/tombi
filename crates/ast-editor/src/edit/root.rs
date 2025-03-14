use futures::FutureExt;

use crate::rule::{root_table_keys_order, TableOrArrayOfTables};

impl crate::Edit for ast::Root {
    fn edit<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            let mut changes = vec![];
            let mut key_values = vec![];
            let mut table_or_array_of_tables = vec![];

            for item in self.items() {
                match item {
                    ast::RootItem::Table(table) => {
                        changes.extend(
                            table
                                .edit(&[], value_schema, schema_url, definitions, schema_context)
                                .await,
                        );
                        table_or_array_of_tables.push(TableOrArrayOfTables::Table(table));
                    }
                    ast::RootItem::ArrayOfTables(array_of_tables) => {
                        changes.extend(
                            array_of_tables
                                .edit(&[], value_schema, schema_url, definitions, schema_context)
                                .await,
                        );
                        table_or_array_of_tables
                            .push(TableOrArrayOfTables::ArrayOfTables(array_of_tables));
                    }
                    ast::RootItem::KeyValue(key_value) => {
                        changes.extend(
                            key_value
                                .edit(&[], value_schema, schema_url, definitions, schema_context)
                                .await,
                        );
                        key_values.push(key_value);
                    }
                };
            }

            if let (Some(value_schema), Some(schema_url), Some(definitions)) =
                (value_schema, schema_url, definitions)
            {
                changes.extend(
                    root_table_keys_order(
                        key_values,
                        table_or_array_of_tables,
                        value_schema,
                        schema_url,
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
