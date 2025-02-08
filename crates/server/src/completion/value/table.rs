use crate::completion::{
    value::{
        all_of::find_all_of_completion_items, any_of::find_any_of_completion_items,
        one_of::find_one_of_completion_items, type_hint_value,
    },
    CompletionCandidate, CompletionContent, CompletionHint, FindCompletionContents,
};
use config::TomlVersion;
use schema_store::{
    Accessor, FindSchemaCandidates, SchemaDefinitions, SchemaUrl, TableSchema, ValueSchema,
};

impl FindCompletionContents for document_tree::Table {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&SchemaUrl>,
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
                            let need_magic_trigger = match completion_hint {
                                Some(
                                    CompletionHint::DotTrigger { range, .. }
                                    | CompletionHint::EqualTrigger { range, .. },
                                ) => range.end() <= key.range().start(),
                                Some(CompletionHint::InArray | CompletionHint::InTableHeader) => {
                                    false
                                }
                                None => true,
                            };
                            if matches!(value, document_tree::Value::Incomplete { .. })
                                && need_magic_trigger
                            {
                                return CompletionContent::new_magic_triggers(
                                    accessor_str,
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
                                let key_name = &property.key().to_string();
                                if !key_name.starts_with(accessor_str) {
                                    continue;
                                }

                                if let Some(value) = self.get(key_name) {
                                    if check_used_table_value(value) {
                                        continue;
                                    }
                                }

                                if let Ok(property_schema) =
                                    property.value_mut().resolve(definitions)
                                {
                                    tracing::trace!("property schema: {:?}", property_schema);
                                    if let Ok(value_schema) =
                                        property.value_mut().resolve(definitions)
                                    {
                                        let Some(contents) = collect_table_key_completion_contents(
                                            self,
                                            table_schema,
                                            key_name,
                                            position,
                                            accessors,
                                            completion_hint,
                                            schema_url,
                                            &value_schema,
                                            definitions,
                                        ) else {
                                            continue;
                                        };
                                        completion_contents.extend(contents);
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
                                    let pattern_property_schema = pattern_property.value_mut();
                                    tracing::trace!(
                                        "pattern property schema: {:?}",
                                        pattern_property_schema
                                    );
                                    if let Ok(value_schema) =
                                        pattern_property_schema.resolve(definitions)
                                    {
                                        return get_property_value_completion_contents(
                                            accessor_str,
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
                                    tracing::trace!(
                                        "additional property schema: {:?}",
                                        additional_property_schema
                                    );

                                    get_property_value_completion_contents(
                                        accessor_str,
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
                                accessor_str,
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
                        let key_name = &property.key().to_string();

                        if let Some(value) = self.get(key_name) {
                            if check_used_table_value(value) {
                                continue;
                            }
                        }

                        if let Ok(value_schema) = property.value_mut().resolve(definitions) {
                            let Some(contents) = collect_table_key_completion_contents(
                                self,
                                table_schema,
                                key_name,
                                position,
                                accessors,
                                completion_hint,
                                schema_url,
                                &value_schema,
                                definitions,
                            ) else {
                                continue;
                            };
                            completion_contents.extend(contents);
                        }
                    }

                    if completion_contents.is_empty() {
                        if let Some(pattern_properties) = &table_schema.pattern_properties {
                            let patterns = pattern_properties
                                .iter()
                                .map(|pattern_property| pattern_property.key().clone())
                                .collect::<Vec<_>>();
                            completion_contents.push(CompletionContent::new_pattern_key(
                                patterns.as_ref(),
                                position,
                                schema_url,
                                completion_hint,
                            ))
                        } else if table_schema.has_additional_property_schema() {
                            completion_contents.push(CompletionContent::new_additional_key(
                                position,
                                schema_url,
                                completion_hint,
                            ));
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
                        get_property_value_completion_contents(
                            accessor_str,
                            value,
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            None,
                            None,
                            completion_hint,
                        )
                    } else {
                        Vec::with_capacity(0)
                    }
                } else {
                    vec![CompletionContent::new_type_hint_empty_key(
                        position,
                        schema_url,
                        completion_hint,
                    )]
                }
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
        schema_url: Option<&SchemaUrl>,
        definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        let Some(definitions) = definitions else {
            unreachable!("definitions must be provided");
        };

        let mut completion_items = Vec::new();

        for mut property in self.properties.iter_mut() {
            let label = &property.key().to_string();

            if let Ok(value_schema) = property.value_mut().resolve(definitions) {
                let (schema_candidates, errors) =
                    value_schema.find_schema_candidates(accessors, definitions);

                for error in errors {
                    tracing::error!("{}", error);
                }

                for schema_candidate in schema_candidates {
                    if let Some(CompletionHint::InTableHeader) = completion_hint {
                        if count_table_or_array_schema(value_schema, definitions) == 0 {
                            continue;
                        }
                    }

                    completion_items.push(CompletionContent::new_key(
                        label,
                        position,
                        schema_candidate.detail(definitions, completion_hint),
                        schema_candidate.documentation(definitions, completion_hint),
                        self.required.as_ref(),
                        schema_url,
                        completion_hint,
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

fn count_table_or_array_schema(
    value_schema: &ValueSchema,
    definitions: &SchemaDefinitions,
) -> usize {
    value_schema
        .match_flattened_schemas(&|schema| {
            matches!(schema, ValueSchema::Table(_) | ValueSchema::Array(_))
        })
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
    schema_url: Option<&SchemaUrl>,
    definitions: Option<&SchemaDefinitions>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    tracing::trace!("accessor_str: {:?}", accessor_str);
    tracing::trace!("value: {:?}", value);
    tracing::trace!("keys: {:?}", keys);
    tracing::trace!("accessors: {:?}", accessors);
    tracing::trace!("value schema: {:?}", value_schema);
    tracing::trace!("completion hint: {:?}", completion_hint);

    if keys.len() == 1 {
        match completion_hint {
            Some(CompletionHint::InArray) => {
                return type_hint_value(Some(accessor_str), position, None, completion_hint)
            }
            Some(CompletionHint::DotTrigger { range } | CompletionHint::EqualTrigger { range }) => {
                let key = keys.first().unwrap();
                if value_schema.is_none() {
                    if range.end() <= key.range().start() {
                        return vec![CompletionContent::new_type_hint_key(
                            accessor_str,
                            text::Range::new(range.end(), position),
                            schema_url,
                            completion_hint,
                        )];
                    }
                    return type_hint_value(Some(accessor_str), position, None, completion_hint);
                }
            }
            Some(CompletionHint::InTableHeader) => {
                if let (Some(value_schema), Some(definitions)) = (value_schema, definitions) {
                    if count_table_or_array_schema(value_schema, definitions) == 0 {
                        return Vec::with_capacity(0);
                    }
                }
            }
            None => {
                if matches!(value, document_tree::Value::Incomplete { .. }) {
                    match completion_hint {
                        Some(
                            CompletionHint::InTableHeader
                            | CompletionHint::DotTrigger { .. }
                            | CompletionHint::EqualTrigger { .. },
                        )
                        | None => {
                            return CompletionContent::new_magic_triggers(
                                accessor_str,
                                position,
                                schema_url,
                            );
                        }
                        Some(CompletionHint::InArray) => {
                            return vec![CompletionContent::new_type_hint_key(
                                accessor_str,
                                text::Range::at(position),
                                schema_url,
                                completion_hint,
                            )];
                        }
                    }
                }
            }
        }
    }
    value.find_completion_contents(
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
    )
}

fn check_used_table_value(value: &document_tree::Value) -> bool {
    match value {
        document_tree::Value::Boolean(_)
        | document_tree::Value::Integer(_)
        | document_tree::Value::Float(_)
        | document_tree::Value::String(_)
        | document_tree::Value::OffsetDateTime(_)
        | document_tree::Value::LocalDateTime(_)
        | document_tree::Value::LocalDate(_)
        | document_tree::Value::LocalTime(_) => return true,
        document_tree::Value::Array(array) => {
            if array.kind() == document_tree::ArrayKind::Array {
                return true;
            }
        }
        document_tree::Value::Table(table) => {
            if table.kind() == document_tree::TableKind::InlineTable {
                return true;
            }
        }
        document_tree::Value::Incomplete { .. } => {}
    }
    false
}

fn collect_table_key_completion_contents(
    table: &document_tree::Table,
    table_schema: &TableSchema,
    key_name: &String,
    position: text::Position,
    accessors: &Vec<Accessor>,
    completion_hint: Option<CompletionHint>,
    schema_url: Option<&SchemaUrl>,
    value_schema: &ValueSchema,
    definitions: &SchemaDefinitions,
) -> Option<Vec<CompletionContent>> {
    let mut completion_contents = Vec::new();

    let (schema_candidates, errors) = value_schema.find_schema_candidates(accessors, definitions);

    for error in errors {
        tracing::error!("{}", error);
    }

    for schema_candidate in schema_candidates {
        match schema_candidate {
            ValueSchema::Boolean(_)
            | ValueSchema::Integer(_)
            | ValueSchema::Float(_)
            | ValueSchema::String(_)
            | ValueSchema::OffsetDateTime(_)
            | ValueSchema::LocalDateTime(_)
            | ValueSchema::LocalDate(_)
            | ValueSchema::LocalTime(_) => {
                if matches!(completion_hint, Some(CompletionHint::InTableHeader))
                    || table.get(key_name).is_some()
                {
                    return None;
                }
            }
            ValueSchema::Array(_) | ValueSchema::Table(_) => {
                if matches!(completion_hint, Some(CompletionHint::InTableHeader))
                    && count_table_or_array_schema(value_schema, definitions) == 0
                {
                    return None;
                }
                if let ValueSchema::Table(table_schema) = value_schema {
                    if !table_schema.additional_properties
                        && !table_schema.has_additional_property_schema()
                        && table_schema.pattern_properties.is_none()
                    {
                        if table_schema.properties.iter().all(|property| {
                            let key_str = &property.key().to_string();
                            table.get(key_str).is_some()
                        }) {
                            return None;
                        }
                    }
                }
            }
            ValueSchema::Null
            | ValueSchema::OneOf(_)
            | ValueSchema::AnyOf(_)
            | ValueSchema::AllOf(_) => {
                unreachable!("Null, OneOf, AnyOf, and AllOf are not allowed in flattened schema");
            }
        }

        completion_contents.push(CompletionContent::new_key(
            key_name,
            position,
            schema_candidate.detail(definitions, completion_hint),
            schema_candidate.documentation(definitions, completion_hint),
            table_schema.required.as_ref(),
            schema_url,
            completion_hint,
        ));
    }

    Some(completion_contents)
}
