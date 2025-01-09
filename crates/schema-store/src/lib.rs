mod accessor;
mod error;
mod json_schema;
mod schema;
mod store;
mod value_type;

pub use accessor::{Accessor, Accessors, Key};
pub use error::Error;
pub use json_schema::{SchemaType, Value, DEFAULT_CATALOG_URL};
pub use schema::DocumentSchema;
pub use schema::{Referable, ValueSchema};
pub use store::SchemaStore;
pub use value_type::ValueType;

#[derive(Debug)]
pub struct KeysValueInfo {
    accessors: Accessors,
    value_type: ValueType,
}

impl KeysValueInfo {
    pub fn accessors(&self) -> &Accessors {
        &self.accessors
    }

    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }
}

pub fn get_keys_value_info(
    root: document_tree::Root,
    keys: &[document_tree::Key],
    position: text::Position,
    toml_version: config::TomlVersion,
) -> Option<KeysValueInfo> {
    let mut accessors = Vec::new();
    let mut value_type = None;
    let table: document_tree::Table = root.into();
    let mut table_ref = &table;

    for key in keys {
        accessors.push(Accessor::Key(key.to_raw_text(toml_version).into()));

        if let Some(value) = table_ref.get(key) {
            if let Some(table) = get_item_table(value, &mut accessors, &mut value_type, position) {
                table_ref = table;
            }
        }
    }

    value_type.map(|value_type| KeysValueInfo {
        accessors: Accessors::new(accessors),
        value_type,
    })
}

fn get_item_table<'a>(
    value: &'a document_tree::Value,
    accessors: &mut Vec<Accessor>,
    value_type: &mut Option<ValueType>,
    position: text::Position,
) -> Option<&'a document_tree::Table> {
    use document_tree::ArrayKind::*;
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
            *value_type = Some(ValueType::Array);

            for (index, value) in array.values().iter().enumerate() {
                if value.range().contains(position) {
                    accessors.push(Accessor::Index(index));
                    let table_ref = get_item_table(value, accessors, value_type, position);

                    match array.kind() {
                        ArrayOfTables | ParentArrayOfTables => {
                            *value_type = Some(ValueType::Array);
                        }
                        Array => {}
                    }

                    return table_ref;
                }
            }
            None
        }
        Value::Table(tbl) => {
            *value_type = Some(ValueType::Table);
            Some(tbl)
        }
    }
}
