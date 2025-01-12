use crate::completion::CompletionCandidate;

use super::{CompletionHint, FindCompletionItems};
use indexmap::map::MutableKeys;
use schema_store::{FindCandidates, TableSchema, ValueSchema};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

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
                        tracing::debug!("key: {}", key);
                        tracing::debug!("schema_candidate: {:?}", schema_candidate);

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
