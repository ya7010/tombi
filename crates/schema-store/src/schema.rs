mod candidate;
mod catalog;
mod document;
mod referable;
mod value;

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
