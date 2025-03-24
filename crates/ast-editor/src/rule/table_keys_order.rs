use std::borrow::Cow;

use ast::AstNode;
use futures::{future::BoxFuture, FutureExt};
use indexmap::IndexMap;
use itertools::Itertools;
use schema_store::{
    AllOfSchema, AnyOfSchema, CurrentSchema, OneOfSchema, SchemaAccessor, SchemaContext,
    ValueSchema,
};
use syntax::SyntaxElement;
use validator::Validate;
use x_tombi::TableKeysOrder;

pub async fn table_keys_order<'a>(
    value: &'a document_tree::Value,
    key_values: Vec<ast::KeyValue>,
    value_schema: Option<&'a ValueSchema>,
    schema_url: Option<&'a schema_store::SchemaUrl>,
    definitions: Option<&'a schema_store::SchemaDefinitions>,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<crate::Change> {
    if key_values.is_empty() {
        return Vec::with_capacity(0);
    }

    let old = std::ops::RangeInclusive::new(
        SyntaxElement::Node(key_values.first().unwrap().syntax().clone()),
        SyntaxElement::Node(key_values.last().unwrap().syntax().clone()),
    );

    let targets = key_values
        .into_iter()
        .map(|kv| {
            (
                kv.keys()
                    .map(|key| {
                        key.keys()
                            .map(|key| {
                                SchemaAccessor::Key(
                                    key.try_to_raw_text(schema_context.toml_version).unwrap(),
                                )
                            })
                            .collect_vec()
                    })
                    .unwrap_or_default(),
                kv,
            )
        })
        .collect_vec();

    let new = sorted_accessors(
        value,
        &[],
        targets,
        value_schema,
        schema_url,
        definitions,
        schema_context,
    )
    .await
    .into_iter()
    .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
    .collect_vec();

    vec![crate::Change::ReplaceRange { old, new }]
}

pub fn sorted_accessors<'a: 'b, 'b, T>(
    value: &'a document_tree::Value,
    validation_accessors: &'a [schema_store::SchemaAccessor],
    targets: Vec<(Vec<schema_store::SchemaAccessor>, T)>,
    value_schema: Option<&'a ValueSchema>,
    schema_url: Option<&'a schema_store::SchemaUrl>,
    definitions: Option<&'a schema_store::SchemaDefinitions>,
    schema_context: &'a SchemaContext<'a>,
) -> BoxFuture<'b, Vec<T>>
where
    T: Send + Clone + std::fmt::Debug + 'b,
{
    async move {
        if let (
            Some(
                ValueSchema::OneOf(OneOfSchema { schemas, .. })
                | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                | ValueSchema::AllOf(AllOfSchema { schemas, .. }),
            ),
            Some(schema_url),
            Some(definitions),
        ) = (value_schema, schema_url, definitions)
        {
            for schema in schemas.write().await.iter_mut() {
                if let Ok(Some(CurrentSchema {
                    schema_url,
                    value_schema,
                    definitions,
                })) = schema
                    .resolve(
                        Cow::Borrowed(schema_url),
                        Cow::Borrowed(definitions),
                        schema_context.store,
                    )
                    .await
                {
                    if value
                        .validate(
                            validation_accessors,
                            Some(value_schema),
                            Some(&schema_url),
                            Some(&definitions),
                            schema_context,
                        )
                        .await
                        .is_ok()
                    {
                        return sorted_accessors(
                            value,
                            validation_accessors,
                            targets.clone(),
                            Some(value_schema),
                            Some(&schema_url),
                            Some(&definitions),
                            schema_context,
                        )
                        .await;
                    }
                }
            }
            return targets.into_iter().map(|(_, target)| target).collect_vec();
        }

        let mut results = Vec::with_capacity(targets.len());
        let mut new_targets_map = IndexMap::new();
        for (accessors, target) in targets {
            if let Some(accessor) = accessors.first() {
                new_targets_map
                    .entry(accessor.clone())
                    .or_insert_with(Vec::new)
                    .push((accessors[1..].to_vec(), target));
            } else {
                results.push(target);
            }
        }

        match (value, value_schema, schema_url, definitions) {
            (
                document_tree::Value::Table(table),
                Some(ValueSchema::Table(table_schema)),
                Some(schema_url),
                Some(definitions),
            ) => {
                if new_targets_map
                    .iter()
                    .all(|(accessor, _)| matches!(accessor, SchemaAccessor::Key(_)))
                {
                    let sorted_targets = match table_schema.keys_order {
                        Some(TableKeysOrder::Ascending) => new_targets_map
                            .into_iter()
                            .sorted_by(|(a_accessor, _), (b_accessor, _)| {
                                a_accessor.partial_cmp(b_accessor).unwrap()
                            })
                            .collect_vec(),
                        Some(TableKeysOrder::Descending) => new_targets_map
                            .into_iter()
                            .sorted_by(|(a_accessor, _), (b_accessor, _)| {
                                b_accessor.partial_cmp(a_accessor).unwrap()
                            })
                            .rev()
                            .collect_vec(),
                        Some(TableKeysOrder::Schema) => {
                            let mut sorted_targets = Vec::with_capacity(new_targets_map.len());

                            for accessor in table_schema.properties.read().await.keys() {
                                if let Some(targets) = new_targets_map.shift_remove(accessor) {
                                    sorted_targets.push((accessor.to_owned(), targets));
                                }
                            }
                            sorted_targets.extend(new_targets_map);
                            sorted_targets
                        }
                        None => new_targets_map.into_iter().collect_vec(),
                    };

                    for (accessor, targets) in sorted_targets {
                        if let Some(value) = table.get(&accessor.to_string()) {
                            if let Some(referable_schema) =
                                table_schema.properties.write().await.get_mut(&accessor)
                            {
                                if let Ok(Some(CurrentSchema {
                                    schema_url,
                                    value_schema,
                                    definitions,
                                })) = referable_schema
                                    .resolve(
                                        Cow::Borrowed(schema_url),
                                        Cow::Borrowed(definitions),
                                        schema_context.store,
                                    )
                                    .await
                                {
                                    results.extend(
                                        sorted_accessors(
                                            value,
                                            &validation_accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor))
                                                .collect_vec(),
                                            targets,
                                            Some(value_schema),
                                            Some(&schema_url),
                                            Some(&definitions),
                                            schema_context,
                                        )
                                        .await,
                                    );
                                } else {
                                    results.extend(targets.into_iter().map(|(_, target)| target));
                                }
                            } else if let Some(referable_schema) =
                                &table_schema.additional_property_schema
                            {
                                if let Ok(Some(CurrentSchema {
                                    schema_url,
                                    value_schema,
                                    definitions,
                                })) = referable_schema
                                    .write()
                                    .await
                                    .resolve(
                                        Cow::Borrowed(schema_url),
                                        Cow::Borrowed(definitions),
                                        schema_context.store,
                                    )
                                    .await
                                {
                                    results.extend(
                                        sorted_accessors(
                                            value,
                                            &validation_accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor))
                                                .collect_vec(),
                                            targets,
                                            Some(value_schema),
                                            Some(&schema_url),
                                            Some(&definitions),
                                            schema_context,
                                        )
                                        .await,
                                    );
                                } else {
                                    results.extend(targets.into_iter().map(|(_, target)| target));
                                }
                            } else {
                                results.extend(targets.into_iter().map(|(_, target)| target));
                            }
                        } else {
                            results.extend(targets.into_iter().map(|(_, target)| target));
                        }
                    }
                    return results;
                }
            }
            (
                document_tree::Value::Array(array),
                Some(ValueSchema::Array(array_schema)),
                Some(schema_url),
                Some(definitions),
            ) => {
                if new_targets_map
                    .iter()
                    .all(|(accessor, _)| matches!(accessor, SchemaAccessor::Index))
                {
                    if let Some(referable_schema) = &array_schema.items {
                        if let Ok(Some(CurrentSchema {
                            schema_url,
                            value_schema,
                            definitions,
                        })) = referable_schema
                            .write()
                            .await
                            .resolve(
                                Cow::Borrowed(schema_url),
                                Cow::Borrowed(definitions),
                                schema_context.store,
                            )
                            .await
                        {
                            for (value, (_, targets)) in array.iter().zip(new_targets_map) {
                                results.extend(
                                    sorted_accessors(
                                        value,
                                        validation_accessors,
                                        targets,
                                        Some(value_schema),
                                        Some(&schema_url),
                                        Some(&definitions),
                                        schema_context,
                                    )
                                    .await,
                                );
                            }
                        } else {
                            for targets in new_targets_map.into_iter().map(|(_, targets)| targets) {
                                results.extend(targets.into_iter().map(|(_, target)| target));
                            }
                        }
                    }

                    return results;
                }
            }
            _ => {}
        }

        for (_, targets) in new_targets_map {
            results.extend(targets.into_iter().map(|(_, target)| target));
        }

        results
    }
    .boxed()
}
