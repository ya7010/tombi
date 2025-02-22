use ahash::AHashMap;

use super::DocumentSchema;

#[derive(Debug, Clone, Default)]
pub struct SourceSchema {
    pub root: Option<DocumentSchema>,
    pub sub_schemas: AHashMap<Vec<String>, DocumentSchema>,
}
