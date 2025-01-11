mod all_of;
mod any_of;
mod array;
mod boolean;
mod candidate;
mod catalog;
mod document;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod one_of;
mod referable;
mod string;
mod table;
mod value;

pub use all_of::AllOfSchema;
pub use any_of::AnyOfSchema;
pub use array::ArraySchema;
pub use boolean::BooleanSchema;
pub use float::FloatSchema;
pub use integer::IntegerSchema;
pub use local_date::LocalDateSchema;
pub use local_date_time::LocalDateTimeSchema;
pub use local_time::LocalTimeSchema;
pub use offset_date_time::OffsetDateTimeSchema;
pub use one_of::OneOfSchema;
pub use string::StringSchema;
pub use table::TableSchema;

use std::sync::{Arc, RwLock};

pub use candidate::FindCandidates;
pub use catalog::CatalogSchema;
pub use document::DocumentSchema;
use indexmap::IndexMap;
pub use referable::Referable;
pub use value::*;

use crate::Accessor;

pub type SchemaProperties = Arc<RwLock<IndexMap<Accessor, Referable<ValueSchema>>>>;
pub type SchemaDefinitions = Arc<RwLock<ahash::HashMap<String, Referable<ValueSchema>>>>;
pub type Schemas = Arc<RwLock<Vec<Referable<ValueSchema>>>>;
