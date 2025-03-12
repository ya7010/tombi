use std::borrow::Cow;

use ahash::AHashMap;
use ast::AstNode;
use futures::{future::BoxFuture, FutureExt};
use itertools::Itertools;
use schema_store::{CurrentSchema, SchemaAccessor, SchemaContext, ValueSchema};
use syntax::SyntaxElement;
use x_tombi::TableKeysOrder;

pub async fn table_keys_order<'a>(
    key_values: Vec<ast::KeyValue>,
    value_schema: &'a ValueSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
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
                    .into_iter()
                    .map(|key| {
                        key.keys().into_iter().map(|key| {
                            SchemaAccessor::Key(
                                key.try_to_raw_text(schema_context.toml_version).unwrap(),
                            )
                        })
                    })
                    .flatten()
                    .collect_vec(),
                kv,
            )
        })
        .collect_vec();

    let ValueSchema::Table(table_schema) = value_schema else {
        return Vec::with_capacity(0);
    };

    let new = sorted_accessors(
        targets,
        &ValueSchema::Table(table_schema.clone()),
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

fn sorted_accessors<'a: 'b, 'b, T>(
    targets: Vec<(Vec<schema_store::SchemaAccessor>, T)>,
    value_schema: &'a ValueSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a SchemaContext<'a>,
) -> BoxFuture<'b, Vec<T>>
where
    T: Send + std::fmt::Debug + 'b,
{
    async move {
        let mut results = Vec::with_capacity(targets.len());
        let mut new_targets_map = AHashMap::new();
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

        match value_schema {
            ValueSchema::Table(table_schema) => {
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
                                if let Some(targets) = new_targets_map.remove(accessor) {
                                    sorted_targets.push((accessor.to_owned(), targets));
                                }
                            }
                            sorted_targets.extend(new_targets_map);
                            sorted_targets
                        }
                        None => new_targets_map.into_iter().collect_vec(),
                    };

                    for (accessor, targets) in sorted_targets {
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
                                        targets,
                                        value_schema,
                                        &schema_url,
                                        &definitions,
                                        schema_context,
                                    )
                                    .await,
                                );
                            }
                        } else {
                            results.extend(targets.into_iter().map(|(_, target)| target));
                        }
                    }
                    return results;
                }
            }
            ValueSchema::Array(array_schema) => {
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
                            for (_, targets) in new_targets_map {
                                results.extend(
                                    sorted_accessors(
                                        targets,
                                        value_schema,
                                        &schema_url,
                                        &definitions,
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
