use crate::completion::{
    value::{
        all_of::find_all_of_completion_items, any_of::find_any_of_completion_items,
        one_of::find_one_of_completion_items, type_hint_value,
    },
    CompletionCandidate, CompletionContent, CompletionEdit, CompletionHint, FindCompletionContents,
};
use config::TomlVersion;
use schema_store::{Accessor, FindSchemaCandidates, SchemaDefinitions, TableSchema, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionContents for document_tree::Table {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        tracing::trace!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value schema: {:?}", value_schema);
        tracing::trace!("completion hint: {:?}", completion_hint);

        match value_schema {
            Some(ValueSchema::Table(table_schema)) => {
                let Some(definitions) = definitions else {
                    unreachable!("definitions must be provided");
                };
                let mut completion_contents = Vec::new();

                if let Some(key) = keys.first() {
                    let accessor_str = &key.to_raw_text(toml_version);
                    if let Some(value) = self.get(key) {
                        let accessor = Accessor::Key(accessor_str.to_string());

                        if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                            if matches!(value, document_tree::Value::Incomplete { .. })
                                && completion_hint.is_none()
                            {
                                return CompletionContent::new_magic_triggers(
                                    &accessor_str,
                                    position,
                                    schema_url,
                                );
                            }

                            if let Ok(property_schema) = property.value_mut().resolve(definitions) {
                                tracing::trace!("property schema: {:?}", property_schema);
                                return value.find_completion_contents(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    Some(property_schema),
                                    toml_version,
                                    position,
                                    &keys[1..],
                                    schema_url,
                                    Some(definitions),
                                    completion_hint,
                                );
                            }
                        } else if keys.len() == 1 {
                            for mut property in table_schema.properties.iter_mut() {
                                let label = property.key().to_string();
                                if !label.starts_with(accessor_str.as_str()) {
                                    continue;
                                }
                                if let Ok(value_schema) = property.value_mut().resolve(definitions)
                                {
                                    let (schema_candidates, errors) =
                                        value_schema.find_schema_candidates(accessors, definitions);

                                    for error in errors {
                                        tracing::error!("{}", error);
                                    }
                                    for schema_candidate in schema_candidates {
                                        if let Some(CompletionHint::InTableHeader) = completion_hint
                                        {
                                            if count_header_table_or_array(
                                                value_schema,
                                                definitions,
                                            ) == 0
                                            {
                                                continue;
                                            }
                                        }
                                        completion_contents.push(CompletionContent::new_property(
                                            label.clone(),
                                            schema_candidate.detail(definitions, completion_hint),
                                            schema_candidate
                                                .documentation(definitions, completion_hint),
                                            table_schema.required.as_ref(),
                                            CompletionEdit::new_propery(
                                                &label,
                                                position,
                                                completion_hint,
                                            ),
                                            schema_url,
                                        ));
                                    }
                                }
                            }
                        }

                        if !completion_contents.is_empty() {
                            return completion_contents;
                        }

                        if let Some(pattern_properties) = &table_schema.pattern_properties {
                            for mut pattern_property in pattern_properties.iter_mut() {
                                let property_key = pattern_property.key();
                                let Ok(pattern) = regex::Regex::new(property_key) else {
                                    tracing::error!(
                                        "Invalid regex pattern property: {}",
                                        property_key
                                    );
                                    continue;
                                };
                                if pattern.is_match(accessor_str) {
                                    let property_schema = pattern_property.value_mut();
                                    if let Ok(value_schema) = property_schema.resolve(definitions) {
                                        return get_property_value_completion_contents(
                                            &accessor_str,
                                            value,
                                            accessors,
                                            Some(value_schema),
                                            toml_version,
                                            position,
                                            keys,
                                            schema_url,
                                            Some(definitions),
                                            completion_hint,
                                        );
                                    }
                                }
                            }
                        }

                        if let Some(completion_items) = table_schema
                            .operate_additional_property_schema(
                                |additional_property_schema| {
                                    get_property_value_completion_contents(
                                        &accessor_str,
                                        value,
                                        accessors,
                                        Some(additional_property_schema),
                                        toml_version,
                                        position,
                                        keys,
                                        schema_url,
                                        Some(definitions),
                                        completion_hint,
                                    )
                                },
                                definitions,
                            )
                        {
                            return completion_items;
                        }

                        if table_schema.additional_properties {
                            return get_property_value_completion_contents(
                                &accessor_str,
                                value,
                                accessors,
                                None,
                                toml_version,
                                position,
                                keys,
                                None,
                                None,
                                completion_hint,
                            );
                        }
                    }
                } else {
                    for mut property in table_schema.properties.iter_mut() {
                        let label = property.key().to_string();
                        let key = self
                            .keys()
                            .last()
                            .filter(|k| label == k.to_raw_text(toml_version));
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
                                            | document_tree::Value::LocalTime(_) => continue,
                                            document_tree::Value::Array(_)
                                            | document_tree::Value::Table(_)
                                            | document_tree::Value::Incomplete { .. } => {}
                                        }
                                    }
                                }

                                completion_contents.push(CompletionContent::new_property(
                                    label.clone(),
                                    schema_candidate.detail(definitions, completion_hint),
                                    schema_candidate.documentation(definitions, completion_hint),
                                    table_schema.required.as_ref(),
                                    CompletionEdit::new_propery(&label, position, completion_hint),
                                    schema_url,
                                ));
                            }
                        }
                    }
                }
                completion_contents
            }
            Some(ValueSchema::OneOf(one_of_schema)) => find_one_of_completion_items(
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
            Some(ValueSchema::AnyOf(any_of_schema)) => find_any_of_completion_items(
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
            Some(ValueSchema::AllOf(all_of_schema)) => find_all_of_completion_items(
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
            Some(_) => Vec::with_capacity(0),
            None => {
                if let Some(key) = keys.first() {
                    let accessor_str = &key.to_raw_text(toml_version);
                    if let Some(value) = self.get(key) {
                        return get_property_value_completion_contents(
                            &accessor_str,
                            value,
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            None,
                            None,
                            completion_hint,
                        );
                    }
                }

                Vec::with_capacity(0)
            }
        }
    }
}

impl FindCompletionContents for TableSchema {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        let Some(definitions) = definitions else {
            unreachable!("definitions must be provided");
        };

        let mut completion_items = Vec::new();

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

                    completion_items.push(CompletionContent::new_property(
                        label.clone(),
                        schema_candidate.detail(definitions, completion_hint),
                        schema_candidate.documentation(definitions, completion_hint),
                        self.required.as_ref(),
                        CompletionEdit::new_propery(&label, position, completion_hint),
                        schema_url,
                    ));
                }
            }
        }

        completion_items.push(CompletionContent::new_type_hint_inline_table(
            position,
            schema_url,
            completion_hint,
        ));

        completion_items
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

fn get_property_value_completion_contents(
    accessor_str: &str,
    value: &document_tree::Value,
    accessors: &Vec<Accessor>,
    value_schema: Option<&ValueSchema>,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: Option<&SchemaDefinitions>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    if keys.len() == 1 {
        match completion_hint {
            Some(
                CompletionHint::InArray
                | CompletionHint::DotTrigger { .. }
                | CompletionHint::EqualTrigger { .. }
                | CompletionHint::SpaceTrigger { .. },
            ) => {
                if value_schema.is_none() {
                    return type_hint_value(position, None, completion_hint);
                }
            }
            Some(CompletionHint::InTableHeader) => {
                if let (Some(value_schema), Some(definitions)) = (value_schema, definitions) {
                    if count_header_table_or_array(value_schema, definitions) == 0 {
                        return Vec::with_capacity(0);
                    }
                }
            }
            None => {
                if matches!(value, document_tree::Value::Incomplete { .. }) {
                    return CompletionContent::new_magic_triggers(
                        &accessor_str,
                        position,
                        schema_url,
                    );
                }
            }
        }
    }
    return value.find_completion_contents(
        &accessors
            .clone()
            .into_iter()
            .chain(std::iter::once(Accessor::Key(accessor_str.to_string())))
            .collect(),
        value_schema,
        toml_version,
        position,
        &keys[1..],
        schema_url,
        definitions,
        completion_hint,
    );
}
