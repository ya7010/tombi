use ahash::AHashMap;

use super::{DocumentSchema, SchemaUrl};
use crate::SchemaAccessor;

pub type SubSchemaUrlMap = AHashMap<Vec<SchemaAccessor>, SchemaUrl>;

#[derive(Debug, Clone)]
pub struct SourceSchema {
    pub root_schema: Option<DocumentSchema>,
    pub sub_schema_url_map: SubSchemaUrlMap,
}
