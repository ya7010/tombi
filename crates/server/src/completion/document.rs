use super::{CompletionHint, FindCompletionItems};
use indexmap::map::MutableKeys;
use schema_store::{Accessor, DocumentSchema, FindCandidates, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, MarkupContent, MarkupKind};

impl FindCompletionItems for DocumentSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
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
                if let Ok(value_schema) = value.resolve(definitions) {
                    let (schema_candidates, schema_errors) =
                        value_schema.find_candidates(accessors, definitions);

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
                            detail: schema_candidate.title().map(ToString::to_string),
                            documentation: schema_candidate.description().map(|description| {
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

        if let Some(referable_value_schema) = properties.get_mut(&accessors[0]) {
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
