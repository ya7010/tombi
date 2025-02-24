use ast::AstNode;
use itertools::{sorted, Itertools};
use schema_store::{TableKeyOrder, TableSchema};
use syntax::SyntaxElement;

pub fn table_key_order(
    node: &syntax::SyntaxNode,
    table_schema: &TableSchema,
) -> Vec<crate::Change> {
    // let key_order = match table_schema {
    //     TableSchema {
    //         key_order: Some(key_order),
    //         ..
    //     } => key_order,
    //     _ => return,
    // };
    let key_order = TableKeyOrder::Ascending;

    let key_values = if let Some(table) = ast::Table::cast(node.clone()) {
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
        TableKeyOrder::Descending => Vec::with_capacity(0),
        TableKeyOrder::Schema => Vec::with_capacity(0),
    }
}
