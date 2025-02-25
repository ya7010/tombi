use ast::AstNode;
use itertools::{sorted, Itertools};
use schema_store::{TableKeyOrder, TableSchema};
use syntax::SyntaxElement;

pub async fn table_keys_order_by(
    node: &syntax::SyntaxNode,
    table_schema: &TableSchema,
) -> Vec<crate::Change> {
    let key_order = match table_schema {
        TableSchema {
            key_order: Some(key_order),
            ..
        } => key_order,
        _ => return Vec::with_capacity(0),
    };

    let mut key_values = if let Some(table) = ast::Table::cast(node.clone()) {
        table.key_values().collect_vec()
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
        TableKeyOrder::Ascending => {
            let sorted_key_values = sorted(key_values)
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange {
                old,
                new: sorted_key_values,
            }]
        }
        TableKeyOrder::Descending => {
            let sorted_key_values = sorted(key_values)
                .rev()
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange {
                old,
                new: sorted_key_values,
            }]
        }
        TableKeyOrder::Schema => {
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
