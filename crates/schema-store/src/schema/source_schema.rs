use ahash::AHashMap;

use super::{DocumentSchema, SchemaUrl};
use crate::SchemaAccessor;

pub type SubSchemaUrlMap = AHashMap<Vec<SchemaAccessor>, SchemaUrl>;

#[derive(Debug, Clone)]
pub struct SourceSchema<'a> {
    pub root_schema: Option<&'a DocumentSchema>,
    pub sub_schema_url_map: SubSchemaUrlMap,
}
