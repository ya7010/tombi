use ast::AstNode;
use itertools::{sorted, Itertools};
use schema_store::{TableKeyOrder, TableSchema, ValueSchema};
use syntax::SyntaxElement;

pub fn table_key_order(node: &syntax::SyntaxNode, value_schema: &ValueSchema) {
    let key_order = match value_schema {
        ValueSchema::Table(TableSchema {
            key_order: Some(key_order),
            ..
        }) => key_order,
        _ => return,
    };

    let key_values = if let Some(table) = ast::Table::cast(node.clone()) {
        table.key_values().collect_vec()
    } else {
        return;
    };

    if key_values.is_empty() {
        return;
    }

    let span = text::Span::new(
        key_values.first().unwrap().syntax().span().start(),
        key_values.last().unwrap().syntax().span().end(),
    );

    match key_order {
        TableKeyOrder::Ascending => {
            let sorted_key_values = sorted(key_values)
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();
            tracing::debug!("sorted_key_values: {:#?}", sorted_key_values);
            node.splice_children(span.into(), sorted_key_values);
        }
        TableKeyOrder::Descending => {}
        TableKeyOrder::Schema => {}
    }
}
