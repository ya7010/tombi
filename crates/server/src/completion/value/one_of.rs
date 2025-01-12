use schema_store::{OneOfSchema, Schemas};

use crate::completion::{
    CompletionCandidate, CompletionHint, CompositeSchema, FindCompletionItems,
};

impl FindCompletionItems for OneOfSchema {
    fn find_completion_items(
        &self,
        accessors: &[schema_store::Accessor],
        definitions: &schema_store::SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        let mut completion_items = Vec::new();
        let mut errors = Vec::new();

        if let Ok(mut schemas) = self.schemas.write() {
            for value_schema in schemas.iter_mut() {
                if let Ok(schema) = value_schema.resolve(definitions) {
                    let (mut inner_completion_items, schema_errors) =
                        schema.find_completion_items(accessors, definitions, completion_hint);

                    for completion_item in &mut inner_completion_items {
                        if completion_item.detail.is_none() {
                            completion_item.detail = self.detail(definitions, completion_hint);
                        }
                        if completion_item.documentation.is_none() {
                            completion_item.documentation =
                                self.documentation(definitions, completion_hint);
                        }
                    }

                    completion_items.extend(inner_completion_items);
                    errors.extend(schema_errors);
                } else {
                    errors.push(schema_store::Error::SchemaLockError);
                }
            }
        }

        (completion_items, errors)
    }
}

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
