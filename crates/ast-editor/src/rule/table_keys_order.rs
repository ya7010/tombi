use std::borrow::Cow;

use ast::AstNode;
use futures::{future::BoxFuture, FutureExt};
use itertools::{sorted, Itertools};
use linter::Validate;
use schema_store::{
    AllOfSchema, AnyOfSchema, CurrentSchema, OneOfSchema, SchemaAccessor, SchemaContext,
    SchemaDefinitions, SchemaUrl, TableSchema, ValueSchema,
};
use syntax::SyntaxElement;
use x_tombi::TableKeysOrder;

pub async fn table_keys_order<'a>(
    value: document_tree::Value,
    header_accessors: &'a [SchemaAccessor],
    key_values: Vec<ast::KeyValue>,
    value_schema: &'a ValueSchema,
    schema_url: &'a SchemaUrl,
    definitions: &'a SchemaDefinitions,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<crate::Change> {
    if key_values.is_empty() {
        return Vec::with_capacity(0);
    }

    let Some(table_schema) = get_table_schema(
        &value,
        header_accessors,
        header_accessors,
        value_schema,
        Cow::Borrowed(schema_url),
        Cow::Borrowed(definitions),
        schema_context,
    )
    .await
    else {
        return Vec::with_capacity(0);
    };

    let Some(key_order) = table_schema.keys_order else {
        return Vec::with_capacity(0);
    };

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
            let mut key_values = key_values;
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

fn get_table_schema<'a: 'b, 'b>(
    value: &'a document_tree::Value,
    accessors: &'a [SchemaAccessor],
    validation_accessors: &'a [SchemaAccessor],
    value_schema: &'a ValueSchema,
    schema_url: Cow<'a, SchemaUrl>,
    definitions: Cow<'a, SchemaDefinitions>,
    schema_context: &'a SchemaContext<'a>,
) -> BoxFuture<'b, Option<TableSchema>> {
    async move {
        match &*value_schema {
            ValueSchema::Table(_) | ValueSchema::Array(_) => {}
            ValueSchema::OneOf(OneOfSchema { schemas, .. })
            | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
            | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                {
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
                            if let Some(table_schema) = get_table_schema(
                                value,
                                accessors,
                                validation_accessors,
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
                }
                return None;
            }
            _ => return None,
        }

        if accessors.is_empty() {
            if let ValueSchema::Table(table_schema) = &*value_schema {
                if value
                    .validate(
                        validation_accessors,
                        Some(&value_schema),
                        Some(&schema_url),
                        Some(&definitions),
                        schema_context,
                    )
                    .await
                    .is_ok()
                {
                    return Some(table_schema.to_owned());
                }
            };
            return None;
        }

        match &accessors[0] {
            SchemaAccessor::Key(key) => match (value, &*value_schema) {
                (document_tree::Value::Table(table), ValueSchema::Table(value_schema)) => {
                    if let (Some(value), Some(referable_property_schema)) = (
                        table.get(&key.to_string()),
                        value_schema
                            .properties
                            .write()
                            .await
                            .get_mut(&SchemaAccessor::Key(key.to_string())),
                    ) {
                        if let Ok(Some(CurrentSchema {
                            value_schema,
                            schema_url,
                            definitions,
                        })) = referable_property_schema
                            .resolve(schema_url, definitions, schema_context.store)
                            .await
                        {
                            return get_table_schema(
                                value,
                                &accessors[1..],
                                validation_accessors,
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
                if let ValueSchema::Array(array_schema) = &*value_schema {
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
                            return get_table_schema(
                                value,
                                &accessors[1..],
                                validation_accessors,
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
