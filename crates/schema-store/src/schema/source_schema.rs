use ahash::AHashMap;

use super::{DocumentSchema, SchemaUrl};

pub type SubSchemaUrlMap = AHashMap<Vec<String>, SchemaUrl>;

#[derive(Debug, Clone, Default)]
pub struct SourceSchema {
    pub root: Option<DocumentSchema>,
    pub sub_schema_url_map: SubSchemaUrlMap,
}
