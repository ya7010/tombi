use ast::AstNode;
use futures::FutureExt;
use schema_store::ValueSchema;

impl crate::Edit for ast::Table {
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

            if self.header().unwrap().to_string() != "dependencies" {
                return changes;
            }
            // let table_schema = match (value_schema, definitions) {
            //     (Some(ValueSchema::Table(ref mut table_schema)), Some(definitions)) => {
            //         for key in self.header().unwrap().keys() {
            //             let Ok(key_text) = key.try_to_raw_text(schema_context.toml_version) else {
            //                 return;
            //             };
            //             if let Some(referable_schema) = table_schema
            //                 .properties
            //                 .read()
            //                 .await
            //                 .get(&Accessor::Key(key_text))
            //             {
            //                 if let Ok((mut new_value_schema, new_schema)) = referable_schema
            //                     .resolve(definitions, schema_context.store)
            //                     .await
            //                 {
            //                     (schema_url, definitions) =
            //                         if let Some((new_schema_url, new_definitions)) = &new_schema {
            //                             (Some(new_schema_url), new_definitions)
            //                         } else {
            //                             (schema_url, definitions)
            //                         };
            //                     match new_value_schema {
            //                         ValueSchema::Table(new_table_schema) => {
            //                             table_schema = new_table_schema;
            //                         }
            //                         _ => {}
            //                     }
            //                 }
            //             }
            //         }
            //         Some(table_schema)
            //     }
            //     _ => None,
            // };
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

            if let Some(ValueSchema::Table(table_schema)) = value_schema {
                changes.extend(crate::rule::table_key_order(self.syntax(), table_schema));
            }

            changes
        }
        .boxed()
    }
}
