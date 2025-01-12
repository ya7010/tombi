use schema_store::{OneOfSchema, Schemas};

use crate::completion::CompositeSchema;

impl CompositeSchema for OneOfSchema {
    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn schemas(&self) -> &Schemas {
        &self.schemas
    }
}
