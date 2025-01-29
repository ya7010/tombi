mod accessor;
mod error;
mod json_schema;
mod schema;
mod store;
mod value_type;

pub use accessor::{Accessor, Accessors};
pub use error::Error;
pub use json_schema::{JsonCatalogSchema, SchemaType, Value, DEFAULT_CATALOG_URL};
pub use schema::*;
pub use store::SchemaStore;
pub use value_type::ValueType;

pub fn get_schema_name(schema_url: &url::Url) -> Option<&str> {
    if let Some(path) = schema_url.path().split('/').last() {
        if !path.is_empty() {
            return Some(path);
        }
    }
    schema_url.host_str()
}
