use schema_store::AllOfSchema;

use crate::completion::{Completion, FindCompletionItems};

impl FindCompletionItems for AllOfSchema {
    fn find_completion_items(
        &self,
        accessors: &[schema_store::Accessor],
        definitions: &schema_store::SchemaDefinitions,
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
                        schema.find_completion_items(accessors, definitions);

                    for completion_item in &mut inner_completion_items {
                        if completion_item.detail.is_none() {
                            completion_item.detail = self.detail();
                        }
                        if completion_item.documentation.is_none() {
                            completion_item.documentation = self.documentation();
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

impl Completion for AllOfSchema {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
