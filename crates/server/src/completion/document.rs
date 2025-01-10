use super::FindCompletionItems;
use indexmap::map::MutableKeys;
use schema_store::{Accessor, DocumentSchema, FindCandidates, SchemaDefinitions};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, MarkupContent, MarkupKind};

impl FindCompletionItems for DocumentSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        let mut completion_items = Vec::new();
        let mut errors = Vec::new();

        let Ok(mut properties) = self.properties.write() else {
            errors.push(schema_store::Error::DocumentLockError {
                schema_url: self.document_url.clone(),
            });
            return (completion_items, errors);
        };

        if accessors.is_empty() {
            for (key, value) in properties.iter_mut2() {
                if let Ok(schema) = value.resolve(definitions) {
                    let (schema_candidates, schema_errors) =
                        schema.find_candidates(accessors, definitions);

                    for candidate in schema_candidates {
                        let completion_item = CompletionItem {
                            label: key.to_string(),
                            kind: Some(CompletionItemKind::PROPERTY),
                            detail: candidate.title().map(ToString::to_string),
                            documentation: candidate.description().map(|description| {
                                tower_lsp::lsp_types::Documentation::MarkupContent(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: description.to_string(),
                                })
                            }),
                            ..Default::default()
                        };
                        completion_items.push(completion_item);
                    }
                    errors.extend(schema_errors);
                }
            }

            return (completion_items, errors);
        }

        if let Some(value) = properties.get_mut(&accessors[0]) {
            if let Ok(schema) = value.resolve(&definitions) {
                return schema.find_completion_items(&accessors[1..], &definitions);
            }
        }

        (completion_items, errors)
    }
}
