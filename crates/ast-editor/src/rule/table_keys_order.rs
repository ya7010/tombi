use ast::AstNode;
use itertools::{sorted, Itertools};
use schema_store::{SchemaContext, TableSchema};
use syntax::SyntaxElement;
use x_tombi::TableKeysOrder;

pub async fn table_keys_order<'a>(
    key_values: Vec<ast::KeyValue>,
    table_schema: &'a TableSchema,
    _schema_context: &'a SchemaContext<'a>,
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
