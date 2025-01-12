use schema_store::{AllOfSchema, Schemas};

use crate::completion::CompositeSchema;

impl CompositeSchema for AllOfSchema {
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
