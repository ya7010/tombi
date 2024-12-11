mod accessor;
mod schema;
mod store;
mod value_type;

pub use accessor::{Accessor, Accessors};
pub use schema::Schema;
pub use store::Store;
pub use value_type::ValueType;

pub fn get_keys_value_info(
    root: document_tree::Root,
    keys: &[document_tree::Key],
    position: text::Position,
) -> Option<(Accessors, ValueType)> {
    let mut accessors = Vec::new();
    let mut value_type = None;
    let table: document_tree::Table = root.into();
    let mut table_ref = &table;

    for key in keys {
        accessors.push(Accessor::Key(key.to_string()));
        if let Some(value) = table_ref.get(key) {
            if let Some(table) = get_item_table(value, &mut accessors, &mut value_type, position) {
                table_ref = table;
            }
        }
    }

    if let Some(value_type) = value_type {
        Some((Accessors::new(accessors), value_type))
    } else {
        None
    }
}

fn get_item_table<'a>(
    value: &'a document_tree::Value,
    accessors: &mut Vec<Accessor>,
    value_type: &mut Option<ValueType>,
    position: text::Position,
) -> Option<&'a document_tree::Table> {
    use document_tree::Value;

    match value {
        Value::Boolean(_) => {
            *value_type = Some(ValueType::Boolean);
            None
        }
        Value::Integer(_) => {
            *value_type = Some(ValueType::Integer);
            None
        }
        Value::Float(_) => {
            *value_type = Some(ValueType::Float);
            None
        }
        Value::String(_) => {
            *value_type = Some(ValueType::String);
            None
        }
        Value::OffsetDateTime(_) => {
            *value_type = Some(ValueType::OffsetDateTime);
            None
        }
        Value::LocalDateTime(_) => {
            *value_type = Some(ValueType::LocalDateTime);
            None
        }
        Value::LocalDate(_) => {
            *value_type = Some(ValueType::LocalDate);
            None
        }
        Value::LocalTime(_) => {
            *value_type = Some(ValueType::LocalTime);
            None
        }
        Value::Array(array) => {
            let mut index = 0;
            *value_type = Some(ValueType::Array);

            for value in array.values() {
                if value.range().contains(position) {
                    accessors.push(Accessor::Index(index));
                    let table_ref = get_item_table(value, accessors, value_type, position);

                    return table_ref;
                }
                index += 1;
            }
            None
        }
        Value::Table(tbl) => {
            *value_type = Some(ValueType::Table);
            Some(tbl)
        }
    }
}
