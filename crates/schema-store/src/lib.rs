mod accessor;
mod arena;
mod error;
pub mod json;
mod schema;
mod store;
mod value_type;

pub use accessor::{Accessor, Accessors, SchemaAccessor};
pub use error::Error;
pub use schema::*;
pub use store::SchemaStore;
pub use value_type::ValueType;

pub fn get_schema_name(schema_url: &SchemaUrl) -> Option<&str> {
    if let Some(path) = schema_url.path().split('/').last() {
        if !path.is_empty() {
            return Some(path);
        }
    }
    schema_url.host_str()
}
