use std::borrow::Cow;

use futures::FutureExt;
use linter::Validate;
use schema_store::{
    AllOfSchema, AnyOfSchema, CurrentSchema, OneOfSchema, SchemaAccessor, ValueSchema,
};

mod array;
mod array_of_tables;
mod inline_table;
mod key_value;
mod root;
mod table;
mod value;

pub trait Edit {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>>;
}

async fn get_schema<'a: 'b, 'b>(
    value: &'a document_tree::Value,
    accessors: &'a [schema_store::SchemaAccessor],
    value_schema: &'a ValueSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a schema_store::SchemaContext<'a>,
) -> Option<ValueSchema> {
    fn inner_get_schema<'a: 'b, 'b>(
        value: &'a document_tree::Value,
        accessors: &'a [schema_store::SchemaAccessor],
        validation_accessors: &'a [schema_store::SchemaAccessor],
        value_schema: &'a ValueSchema,
        schema_url: Cow<'a, schema_store::SchemaUrl>,
        definitions: Cow<'a, schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Option<ValueSchema>> {
        async move {
            match &*value_schema {
                ValueSchema::Table(_) | ValueSchema::Array(_) => {}
                ValueSchema::OneOf(OneOfSchema { schemas, .. })
                | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
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
                            return inner_get_schema(
                                value,
                                accessors,
                                validation_accessors,
                                value_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                    }

                    return None;
                }
                _ => return None,
            }

            if accessors.is_empty() {
                return value
                    .validate(
                        validation_accessors,
                        Some(&value_schema),
                        Some(&schema_url),
                        Some(&definitions),
                        schema_context,
                    )
                    .await
                    .ok()
                    .map(|_| value_schema.clone());
            }

            match &accessors[0] {
                SchemaAccessor::Key(key) => match (value, &*value_schema) {
                    (document_tree::Value::Table(table), ValueSchema::Table(table_schema)) => {
                        if let Some(value) = table.get(&key.to_string()) {
                            if let Some(referable_property_schema) = table_schema
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
                                    .resolve(
                                        schema_url.clone(),
                                        definitions.clone(),
                                        schema_context.store,
                                    )
                                    .await
                                {
                                    return inner_get_schema(
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
                            if let Some(pattern_properties) = &table_schema.pattern_properties {
                                for (property_key, pattern_property) in
                                    pattern_properties.write().await.iter_mut()
                                {
                                    if let Ok(pattern) = regex::Regex::new(property_key) {
                                        if pattern.is_match(&key.to_string()) {
                                            if let Ok(Some(CurrentSchema {
                                                schema_url,
                                                value_schema,
                                                definitions,
                                            })) = pattern_property
                                                .resolve(
                                                    schema_url.clone(),
                                                    definitions.clone(),
                                                    schema_context.store,
                                                )
                                                .await
                                            {
                                                return inner_get_schema(
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
                                    } else {
                                        tracing::error!(
                                            "Invalid regex pattern property: {}",
                                            property_key
                                        );
                                    };
                                }
                            }
                            if let Some(additional_properties_schema) =
                                &table_schema.additional_property_schema
                            {
                                if let Ok(Some(CurrentSchema {
                                    value_schema,
                                    schema_url,
                                    definitions,
                                })) = additional_properties_schema
                                    .write()
                                    .await
                                    .resolve(
                                        schema_url.clone(),
                                        definitions.clone(),
                                        schema_context.store,
                                    )
                                    .await
                                {
                                    return inner_get_schema(
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
                    (document_tree::Value::Array(array), ValueSchema::Array(array_schema)) => {
                        if let Some(value) = array.first() {
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
                                    return inner_get_schema(
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
                        } else {
                            return None;
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
                                return inner_get_schema(
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

    inner_get_schema(
        value,
        accessors,
        accessors,
        value_schema,
        Cow::Borrowed(schema_url),
        Cow::Borrowed(definitions),
        schema_context,
    )
    .await
}
