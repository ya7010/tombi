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
mod schema_context;
mod source_schema;
mod string_schema;
mod table_schema;
mod value_schema;

use crate::{Accessor, SchemaStore};
pub use all_of_schema::AllOfSchema;
pub use any_of_schema::AnyOfSchema;
pub use array_schema::ArraySchema;
pub use boolean_schema::BooleanSchema;
pub use catalog_schema::CatalogSchema;
pub use document_schema::DocumentSchema;
pub use float_schema::FloatSchema;
use futures::future::BoxFuture;
pub use integer_schema::IntegerSchema;
pub use local_date_schema::LocalDateSchema;
pub use local_date_time_schema::LocalDateTimeSchema;
pub use local_time_schema::LocalTimeSchema;
pub use offset_date_time_schema::OffsetDateTimeSchema;
pub use one_of_schema::OneOfSchema;
pub use referable_schema::{is_online_url, CurrentSchema, Referable};
pub use schema_context::SchemaContext;
pub use source_schema::SourceSchema;
pub use source_schema::SubSchemaUrlMap;
use std::sync::Arc;
pub use string_schema::StringSchema;
pub use table_schema::{TableKeyOrder, TableSchema};
pub use value_schema::*;

pub type SchemaProperties =
    Arc<tokio::sync::RwLock<ahash::AHashMap<Accessor, Referable<ValueSchema>>>>;
pub type SchemaPatternProperties =
    Arc<tokio::sync::RwLock<ahash::AHashMap<String, Referable<ValueSchema>>>>;
pub type SchemaItemTokio = Arc<tokio::sync::RwLock<Referable<ValueSchema>>>;
pub type SchemaDefinitions =
    Arc<tokio::sync::RwLock<ahash::AHashMap<String, Referable<ValueSchema>>>>;
pub type Schemas = Arc<tokio::sync::RwLock<Vec<Referable<ValueSchema>>>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct SchemaUrl(url::Url);

impl SchemaUrl {
    #[inline]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }

    #[inline]
    pub fn parse(url: &str) -> Result<Self, crate::Error> {
        match url::Url::parse(url) {
            Ok(url) => Ok(Self(url)),
            Err(_) => Err(crate::Error::InvalidSchemaUrl {
                schema_url: url.to_string(),
            }),
        }
    }

    #[inline]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, crate::Error> {
        match url::Url::from_file_path(&path) {
            Ok(url) => Ok(Self(url)),
            Err(_) => Err(crate::Error::InvalidSchemaUrl {
                schema_url: path.as_ref().to_string_lossy().to_string(),
            }),
        }
    }
}

impl std::ops::Deref for SchemaUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for SchemaUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait FindSchemaCandidates {
    fn find_schema_candidates<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a SchemaStore,
    ) -> BoxFuture<'b, (Vec<ValueSchema>, Vec<crate::Error>)>;
}
