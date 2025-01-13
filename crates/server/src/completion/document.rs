use crate::completion::CompletionCandidate;

use super::{CompletionHint, FindCompletionItems};
use schema_store::{
    Accessor, DocumentSchema, Schema, SchemaDefinitions, ValueSchema,
};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

impl FindCompletionItems for DocumentSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        let mut completion_items = Vec::new();
        let mut errors = Vec::new();

        if accessors.is_empty() {
            for mut property in self.properties.iter_mut() {
                let key = property.key().to_string();
                if let Ok(value_schema) = property.value_mut().resolve(definitions) {
                    let (schema_candidates, schema_errors) =
                        value_schema.find_schema_candidates(accessors, definitions);

                    for schema_candidate in schema_candidates {
                        match completion_hint {
                            Some(CompletionHint::InTableHeader) => {
                                if !value_schema.is_match(&|s| {
                                    matches!(s, ValueSchema::Table(_) | ValueSchema::Array(_))
                                }) {
                                    continue;
                                }
                            }
                            _ => {}
                        }

                        let completion_item = CompletionItem {
                            label: key.to_string(),
                            kind: Some(CompletionItemKind::PROPERTY),
                            detail: schema_candidate.detail(definitions, completion_hint),
                            documentation: schema_candidate
                                .documentation(definitions, completion_hint),
                            ..Default::default()
                        };
                        completion_items.push(completion_item);
                    }
                    errors.extend(schema_errors);
                }
            }

            return (completion_items, errors);
        }

        if let Some(mut referable_value_schema) = self.properties.get_mut(&accessors[0]) {
            if let Ok(value_schema) = referable_value_schema.resolve(&definitions) {
                return value_schema.find_completion_items(
                    &accessors[1..],
                    &definitions,
                    completion_hint,
                );
            }
        }

        (completion_items, errors)
    }
}
