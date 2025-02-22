use ahash::AHashMap;

use crate::SchemaAccessor;

use super::{DocumentSchema, SchemaUrl};

pub type SubSchemaUrlMap = AHashMap<Vec<SchemaAccessor>, SchemaUrl>;

#[derive(Debug, Clone)]
pub struct SourceSchema {
    pub root: Option<DocumentSchema>,
    pub sub_schema_url_map: SubSchemaUrlMap,
}
