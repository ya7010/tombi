use super::{CompletionHint, FindCompletionItems};
use indexmap::map::MutableKeys;
use schema_store::{FindCandidates, TableSchema, ValueSchema};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, MarkupContent, MarkupKind};

impl FindCompletionItems for TableSchema {
    fn find_completion_items(
        &self,
        accessors: &[schema_store::Accessor],
        definitions: &schema_store::SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        let mut completions = Vec::new();
        let mut errors = Vec::new();

        let Ok(mut properties) = self.properties.write() else {
            errors.push(schema_store::Error::SchemaLockError);
            return (completions, errors);
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
                        completions.push(completion_item);
                    }
                    errors.extend(schema_errors);
                }
            }

            return (completions, errors);
        }

        if let Some(value) = properties.get_mut(&accessors[0]) {
            if let Ok(schema) = value.resolve(&definitions) {
                return schema.find_completion_items(
                    &accessors[1..],
                    &definitions,
                    completion_hint,
                );
            }
        }

        (completions, errors)
    }
}
