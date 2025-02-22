use ahash::AHashMap;

use super::{DocumentSchema, SchemaUrl};

pub type SubSchemaUrls = AHashMap<Vec<String>, SchemaUrl>;

#[derive(Debug, Clone, Default)]
pub struct SourceSchema {
    pub root: Option<DocumentSchema>,
    pub sub_schema_urls: SubSchemaUrls,
}
