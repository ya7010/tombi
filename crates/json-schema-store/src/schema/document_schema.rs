use crate::Accessor;

use super::value_schema::ValueSchema;

#[derive(Debug, Default)]
pub struct DocumentSchema {
    properties: ahash::HashMap<Accessor, ValueSchema>,
    definitions: ahash::HashMap<String, ValueSchema>,
}

impl DocumentSchema {
    pub fn properties(&self) -> &ahash::HashMap<Accessor, ValueSchema> {
        &self.properties
    }

    pub fn definitions(&self) -> &ahash::HashMap<String, ValueSchema> {
        &self.definitions
    }

    pub fn insert_property(&mut self, accessor: Accessor, schema: ValueSchema) {
        self.properties.insert(accessor, schema);
    }

    pub fn insert_definition(&mut self, name: String, schema: ValueSchema) {
        self.definitions.insert(name, schema);
    }
}
