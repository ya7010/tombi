use std::borrow::Cow;

use ast::AstNode;
use futures::{future::BoxFuture, FutureExt};
use itertools::{sorted, Itertools};
use schema_store::{
    Accessor, AllOfSchema, AnyOfSchema, CurrentSchema, OneOfSchema, SchemaAccessor, SchemaContext,
    SchemaDefinitions, SchemaUrl, TableKeysOrder, TableSchema, ValueSchema,
};
use syntax::SyntaxElement;

pub async fn table_keys_order<'a>(
    value: document_tree::Value,
    accessors: &'a [Accessor],
    key_values: &'a [ast::KeyValue],
    value_schema: &'a ValueSchema,
    schema_url: &'a SchemaUrl,
    definitions: &'a SchemaDefinitions,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<crate::Change> {
    let Some(key_order) = get_table_keys_order(
        &value,
        accessors,
        value_schema,
        Cow::Borrowed(schema_url),
        Cow::Borrowed(definitions),
        schema_context,
    )
    .await
    else {
        return Vec::with_capacity(0);
    };

    let (table_schema, mut key_values) = if let Some(table) = ast::Table::cast(node.clone()) {
        let array_of_tables_keys = table
            .array_of_tables_keys()
            .map(|keys| keys.into_iter().collect_vec())
            .collect_vec();

        let mut accessors = vec![];
        let mut header_keys = vec![];
        for key in table.header().unwrap().keys() {
            accessors.push(SchemaAccessor::Key(
                key.try_to_raw_text(schema_context.toml_version).unwrap(),
            ));
            header_keys.push(key);

            if array_of_tables_keys.contains(&header_keys) {
                accessors.push(SchemaAccessor::Index);
            }
        }

        (table_schema, table.key_values().collect_vec())
    } else if let Some(array_of_table) = ast::ArrayOfTables::cast(node.clone()) {
        (table_schema, array_of_table.key_values().collect_vec())
    } else {
        return Vec::with_capacity(0);
    };

    if key_values.is_empty() {
        return Vec::with_capacity(0);
    }

    let old = std::ops::RangeInclusive::new(
        SyntaxElement::Node(key_values.first().unwrap().syntax().clone()),
        SyntaxElement::Node(key_values.last().unwrap().syntax().clone()),
    );

    match key_order {
        TableKeysOrder::Ascending => {
            let new = sorted(key_values)
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange { old, new }]
        }
        TableKeysOrder::Descending => {
            let new = sorted(key_values)
                .rev()
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange { old, new }]
        }
        TableKeysOrder::Schema => {
            let mut new = vec![];
            for (accessor, _) in table_schema.properties.write().await.iter_mut() {
                key_values = key_values
                    .into_iter()
                    .filter_map(|kv| {
                        if kv.keys().iter().next().map(ToString::to_string)
                            == Some(accessor.to_string())
                        {
                            new.push(SyntaxElement::Node(kv.syntax().clone()));
                            None
                        } else {
                            Some(kv)
                        }
                    })
                    .collect_vec();
            }
            new.extend(
                key_values
                    .into_iter()
                    .map(|kv| SyntaxElement::Node(kv.syntax().clone())),
            );

            vec![crate::Change::ReplaceRange { old, new }]
        }
    }
}

fn get_table_keys_order<'a: 'b, 'b>(
    value: &'a document_tree::Value,
    accessors: &'a [Accessor],
    value_schema: &'a ValueSchema,
    schema_url: Cow<'a, SchemaUrl>,
    definitions: Cow<'a, SchemaDefinitions>,
    schema_context: &'a SchemaContext<'a>,
) -> BoxFuture<'b, Option<Cow<'a, TableSchema>>> {
    async move {
        match value_schema {
            ValueSchema::Table(_) | ValueSchema::Array(_) => {}
            ValueSchema::OneOf(OneOfSchema { schemas, .. })
            | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
            | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                let mut result = None;
                for referable_schema in schemas.write().await.iter_mut() {
                    if let Ok(Some(CurrentSchema {
                        value_schema,
                        schema_url,
                        definitions,
                    })) = referable_schema
                        .resolve(
                            schema_url.clone(),
                            definitions.clone(),
                            schema_context.store,
                        )
                        .await
                    {
                        if let Some(table_schema) = get_table_keys_order(
                            value,
                            &accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                        {
                            return Some(table_schema);
                        }
                    }
                }
                return None;
            }
            _ => return None,
        }

        if accessors.is_empty() {
            match value_schema {
                ValueSchema::Table(table_schema) => {
                    if value.validate(schema_context.toml_version) {
                        return Some(Cow::Borrowed(table_schema));
                    }
                }
                _ => {}
            };
            return None;
        }

        match &accessors[0] {
            Accessor::Key(key) => match value_schema {
                ValueSchema::Table(value_schema) => {
                    if let Some(referable_property_schema) = value_schema
                        .properties
                        .write()
                        .await
                        .get_mut(&SchemaAccessor::Key(key.to_string()))
                    {
                        if let Ok(Some(CurrentSchema {
                            value_schema,
                            schema_url,
                            definitions,
                        })) = referable_property_schema
                            .resolve(schema_url, definitions, schema_context.store)
                            .await
                        {
                            return get_table_keys_order(
                                &accessors[1..],
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                    }
                }
                _ => return None,
            },
            SchemaAccessor::Index => {
                if let ValueSchema::Array(array_schema) = value_schema {
                    if let Some(item_schema) = &array_schema.items {
                        if let Ok(Some(CurrentSchema {
                            value_schema,
                            schema_url,
                            definitions,
                        })) = item_schema
                            .write()
                            .await
                            .resolve(schema_url, definitions, schema_context.store)
                            .await
                        {
                            return get_table_keys_order(
                                &accessors[1..],
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                    }
                }
            }
        }

        None
    }
    .boxed()
}
