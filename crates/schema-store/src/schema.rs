mod all_of_schema;
mod any_of_schema;
mod array_schema;
mod boolean_schema;
mod catalog_schema;
mod document_schema;
mod float_schema;
mod integer_schema;
mod local_date_schema;
mod local_date_time_schema;
mod local_time_schema;
mod offset_date_time_schema;
mod one_of_schema;
mod referable_schema;
mod string_schema;
mod table_schema;
mod value_schema;

pub use all_of_schema::AllOfSchema;
pub use any_of_schema::AnyOfSchema;
pub use array_schema::ArraySchema;
pub use boolean_schema::BooleanSchema;
pub use float_schema::FloatSchema;
pub use integer_schema::IntegerSchema;
pub use local_date_schema::LocalDateSchema;
pub use local_date_time_schema::LocalDateTimeSchema;
pub use local_time_schema::LocalTimeSchema;
pub use offset_date_time_schema::OffsetDateTimeSchema;
pub use one_of_schema::OneOfSchema;
pub use string_schema::StringSchema;
pub use table_schema::TableSchema;

use std::sync::{Arc, RwLock};

pub use catalog_schema::CatalogSchema;
pub use document_schema::DocumentSchema;
pub use referable_schema::Referable;
pub use value_schema::*;

use crate::Accessor;

pub type SchemaProperties = dashmap::DashMap<Accessor, Referable<ValueSchema>>;
pub type SchemaDefinitions = dashmap::DashMap<String, Referable<ValueSchema>>;
pub type Schemas = Arc<RwLock<Vec<Referable<ValueSchema>>>>;

pub trait Schema {
    fn find_schema_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>);
}
