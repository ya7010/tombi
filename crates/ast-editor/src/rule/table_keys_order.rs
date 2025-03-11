use ahash::AHashMap;
use ast::AstNode;
use itertools::Itertools;
use schema_store::{SchemaContext, TableSchema};
use syntax::SyntaxElement;
use x_tombi::TableKeysOrder;

pub async fn table_keys_order<'a>(
    key_values: Vec<ast::KeyValue>,
    table_schema: &'a TableSchema,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<crate::Change> {
    if key_values.is_empty() {
        return Vec::with_capacity(0);
    }

    let Some(keys_order) = table_schema.keys_order else {
        return Vec::with_capacity(0);
    };

    let old = std::ops::RangeInclusive::new(
        SyntaxElement::Node(key_values.first().unwrap().syntax().clone()),
        SyntaxElement::Node(key_values.last().unwrap().syntax().clone()),
    );

    match keys_order {
        TableKeysOrder::Ascending => {
            let new = key_values
                .iter()
                .sorted_by(|a, b| {
                    let a_keys = a.keys().unwrap();
                    let b_keys = b.keys().unwrap();
                    let a_first_key = a_keys.keys().next().unwrap();
                    let b_first_key = b_keys.keys().next().unwrap();
                    a_first_key
                        .try_to_raw_text(schema_context.toml_version)
                        .unwrap()
                        .cmp(
                            &b_first_key
                                .try_to_raw_text(schema_context.toml_version)
                                .unwrap(),
                        )
                })
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange { old, new }]
        }
        TableKeysOrder::Descending => {
            let new = key_values
                .iter()
                .sorted_by(|a, b| {
                    let a_keys = a.keys().unwrap();
                    let b_keys = b.keys().unwrap();
                    let a_first_key = a_keys.keys().next().unwrap();
                    let b_first_key = b_keys.keys().next().unwrap();
                    a_first_key
                        .try_to_raw_text(schema_context.toml_version)
                        .unwrap()
                        .cmp(
                            &b_first_key
                                .try_to_raw_text(schema_context.toml_version)
                                .unwrap(),
                        )
                })
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
                        if kv.keys().and_then(|keys| {
                            keys.keys().next().and_then(|key| {
                                key.try_to_raw_text(schema_context.toml_version).ok()
                            })
                        }) == Some(accessor.to_string())
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

async fn sorted_keys<'a, T>(
    targets: Vec<(Vec<schema_store::SchemaAccessor>, Vec<T>)>,
    value_schema: &'a TableSchema,
    schema_context: &'a SchemaContext<'a>,
) -> Vec<T> {
    let mut new_targets = AHashMap::new();
    for (accessors, target) in targets {
        if let Some(accessor) = accessors.first() {
            new_targets
                .entry(accessor.clone())
                .or_insert_with(Vec::new)
                .push((accessors[1..].to_vec(), target));
        }
    }

    for (key, targets) in new_targets {
        value_schema.properties.write().await.get(key)
    }

    result
}
