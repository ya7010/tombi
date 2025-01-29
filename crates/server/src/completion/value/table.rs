use crate::completion::{
    find_any_of_completion_items, find_one_of_completion_items, CompletionCandidate,
    CompletionContent,
};

use super::{find_all_if_completion_items, CompletionHint, FindCompletionContents};
use config::TomlVersion;
use schema_store::{Accessor, FindSchemaCandidates, SchemaDefinitions, TableSchema, ValueSchema};
use tower_lsp::lsp_types::{CompletionItemKind, Url};

impl FindCompletionContents for document_tree::Table {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match value_schema {
            ValueSchema::Table(table_schema) => {
                let mut completions = Vec::new();

                if let Some(key) = keys.first() {
                    let accessor = Accessor::Key(key.to_raw_text(toml_version));
                    if let Some(value) = self.get(key) {
                        match value_schema {
                            ValueSchema::Table(table_schema) => {
                                if let Some(mut property) =
                                    table_schema.properties.get_mut(&accessor)
                                {
                                    if let Ok(property_schema) =
                                        property.value_mut().resolve(definitions)
                                    {
                                        return value.find_completion_contents(
                                            &accessors
                                                .clone()
                                                .into_iter()
                                                .chain(std::iter::once(accessor))
                                                .collect(),
                                            property_schema,
                                            toml_version,
                                            position,
                                            &keys[1..],
                                            schema_url,
                                            definitions,
                                            completion_hint,
                                        );
                                    }
                                }
                            }
                            ValueSchema::OneOf(one_of_schema) => {
                                return find_one_of_completion_items(
                                    self,
                                    accessors,
                                    one_of_schema,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    completion_hint,
                                );
                            }
                            ValueSchema::AnyOf(any_of_schema) => {
                                return find_any_of_completion_items(
                                    self,
                                    accessors,
                                    any_of_schema,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    completion_hint,
                                );
                            }
                            ValueSchema::AllOf(all_of_schema) => {
                                return find_all_if_completion_items(
                                    self,
                                    accessors,
                                    all_of_schema,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    completion_hint,
                                );
                            }
                            _ => {}
                        }
                    }
                } else {
                    for mut property in table_schema.properties.iter_mut() {
                        let label = property.key().to_string();
                        let key = self.keys().find(|k| k.to_raw_text(toml_version) == label);
                        if let Ok(value_schema) = property.value_mut().resolve(definitions) {
                            let (schema_candidates, errors) =
                                value_schema.find_schema_candidates(accessors, definitions);

                            for error in errors {
                                tracing::error!("{}", error);
                            }
                            for schema_candidate in schema_candidates {
                                if let Some(CompletionHint::InTableHeader) = completion_hint {
                                    if count_header_table_or_array(value_schema, definitions) == 0 {
                                        continue;
                                    }
                                }
                                if let Some(key) = key {
                                    if let Some(value) = self.get(key) {
                                        match value {
                                            document_tree::Value::Boolean(_)
                                            | document_tree::Value::Integer(_)
                                            | document_tree::Value::Float(_)
                                            | document_tree::Value::String(_)
                                            | document_tree::Value::OffsetDateTime(_)
                                            | document_tree::Value::LocalDateTime(_)
                                            | document_tree::Value::LocalDate(_)
                                            | document_tree::Value::LocalTime(_)
                                            | document_tree::Value::Array(_) => continue,
                                            document_tree::Value::Table(_)
                                            | document_tree::Value::Incomplete { .. } => {}
                                        }
                                    }
                                }

                                let completion_content = CompletionContent {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::PROPERTY),
                                    detail: schema_candidate.detail(definitions, completion_hint),
                                    documentation: schema_candidate
                                        .documentation(definitions, completion_hint),
                                    schema_url: schema_url.cloned(),
                                    ..Default::default()
                                };
                                completions.push(completion_content);
                            }
                        }
                    }
                }
                completions
            }
            ValueSchema::OneOf(one_of_schema) => find_one_of_completion_items(
                self,
                accessors,
                one_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AnyOf(any_of_schema) => find_any_of_completion_items(
                self,
                accessors,
                any_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AllOf(all_of_schema) => find_all_if_completion_items(
                self,
                accessors,
                all_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            _ => Vec::with_capacity(0),
        }
    }
}

impl FindCompletionContents for TableSchema {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        _value_schema: &ValueSchema,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        let mut completions = Vec::new();

        for mut property in self.properties.iter_mut() {
            let label = property.key().to_string();

            if let Ok(value_schema) = property.value_mut().resolve(definitions) {
                let (schema_candidates, errors) =
                    value_schema.find_schema_candidates(accessors, definitions);

                for error in errors {
                    tracing::error!("{}", error);
                }

                for schema_candidate in schema_candidates {
                    if let Some(CompletionHint::InTableHeader) = completion_hint {
                        if count_header_table_or_array(value_schema, definitions) == 0 {
                            continue;
                        }
                    }
                    let completion_content = CompletionContent {
                        label: label.clone(),
                        kind: Some(CompletionItemKind::PROPERTY),
                        detail: schema_candidate.detail(definitions, completion_hint),
                        documentation: schema_candidate.documentation(definitions, completion_hint),
                        schema_url: schema_url.cloned(),
                        ..Default::default()
                    };
                    completions.push(completion_content);
                }
            }
        }
        completions
    }
}

fn table_or_array(value_schema: &ValueSchema) -> bool {
    matches!(value_schema, ValueSchema::Table(_) | ValueSchema::Array(_))
}

fn count_header_table_or_array(
    value_schema: &ValueSchema,
    definitions: &SchemaDefinitions,
) -> usize {
    value_schema
        .match_schemas(&table_or_array)
        .into_iter()
        .filter(|schema| match schema {
            ValueSchema::Array(array_schema) => array_schema
                .operate_item(
                    |item_schema| {
                        item_schema.is_match(&|schema| matches!(schema, ValueSchema::Table(_)))
                    },
                    definitions,
                )
                .unwrap_or(true),
            ValueSchema::Table(_) => true,
            _ => unreachable!("only table and array are allowed"),
        })
        .count()
}
