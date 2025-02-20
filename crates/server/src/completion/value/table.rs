use crate::completion::{
    value::{
        all_of::find_all_of_completion_items, any_of::find_any_of_completion_items,
        one_of::find_one_of_completion_items, type_hint_value,
    },
    CompletionCandidate, CompletionContent, CompletionHint, FindCompletionContents,
};
use config::TomlVersion;
use futures::{
    future::{join_all, BoxFuture},
    FutureExt,
};
use schema_store::{
    is_online_url, Accessor, FindSchemaCandidates, Referable, SchemaDefinitions, SchemaStore,
    SchemaUrl, TableSchema, ValueSchema,
};

impl FindCompletionContents for document_tree::Table {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        tracing::trace!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value schema: {:?}", value_schema);
        tracing::trace!("completion hint: {:?}", completion_hint);

        async move {
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

                            if let Some(property) =
                                table_schema.properties.write().await.get_mut(&accessor)
                            {
                                let need_magic_trigger = match completion_hint {
                                    Some(
                                        CompletionHint::DotTrigger { range, .. }
                                        | CompletionHint::EqualTrigger { range, .. },
                                    ) => range.end() <= key.range().start(),
                                    Some(
                                        CompletionHint::InArray | CompletionHint::InTableHeader,
                                    ) => false,
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

                                if let Ok((property_schema, new_schema)) =
                                    property.resolve(definitions, schema_store).await
                                {
                                    tracing::trace!("property schema: {:?}", property_schema);
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), Some(definitions))
                                        } else {
                                            (schema_url, Some(definitions))
                                        };

                                    return value
                                        .find_completion_contents(
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
                                            definitions,
                                            schema_store,
                                            completion_hint,
                                        )
                                        .await;
                                }
                            } else if keys.len() == 1 {
                                for (key, property) in
                                    table_schema.properties.write().await.iter_mut()
                                {
                                    let key_name = &key.to_string();
                                    if !key_name.starts_with(accessor_str) {
                                        continue;
                                    }

                                    if let Some(value) = self.get(key_name) {
                                        if check_used_table_value(value) {
                                            continue;
                                        }
                                    }

                                    if let Ok((property_schema, new_schema)) =
                                        property.resolve(definitions, schema_store).await
                                    {
                                        tracing::trace!("property schema: {:?}", property_schema);
                                        let (schema_url, definitions) =
                                            if let Some((schema_url, definitions)) = &new_schema {
                                                (Some(schema_url), definitions)
                                            } else {
                                                (schema_url, definitions)
                                            };

                                        let Some(contents) = collect_table_key_completion_contents(
                                            self,
                                            table_schema,
                                            key_name,
                                            position,
                                            accessors,
                                            completion_hint,
                                            schema_url,
                                            property_schema,
                                            definitions,
                                            schema_store,
                                        )
                                        .await
                                        else {
                                            continue;
                                        };
                                        completion_contents.extend(contents);
                                    }
                                }
                            }

                            if !completion_contents.is_empty() {
                                return completion_contents;
                            }

                            if let Some(pattern_properties) = &table_schema.pattern_properties {
                                for (property_key, pattern_property_schema) in
                                    pattern_properties.write().await.iter_mut()
                                {
                                    let Ok(pattern) = regex::Regex::new(property_key) else {
                                        tracing::error!(
                                            "Invalid regex pattern property: {}",
                                            property_key
                                        );
                                        continue;
                                    };
                                    if pattern.is_match(accessor_str) {
                                        tracing::trace!(
                                            "pattern property schema: {:?}",
                                            pattern_property_schema
                                        );
                                        if let Ok((value_schema, new_schema)) =
                                            pattern_property_schema
                                                .resolve(definitions, schema_store)
                                                .await
                                        {
                                            let (schema_url, definitions) = if let Some((
                                                schema_url,
                                                definitions,
                                            )) = &new_schema
                                            {
                                                (Some(schema_url), Some(definitions))
                                            } else {
                                                (schema_url, Some(definitions))
                                            };
                                            return get_property_value_completion_contents(
                                                key,
                                                value,
                                                accessors,
                                                Some(value_schema),
                                                toml_version,
                                                position,
                                                keys,
                                                schema_url,
                                                definitions,
                                                schema_store,
                                                completion_hint,
                                            )
                                            .await;
                                        }
                                    }
                                }
                            }

                            if let Some(referable_additional_property_schema) =
                                &table_schema.additional_property_schema
                            {
                                tracing::trace!(
                                    "additional property schema: {:?}",
                                    referable_additional_property_schema
                                );

                                if let Ok((additional_property_schema, new_schema)) =
                                    referable_additional_property_schema
                                        .write()
                                        .await
                                        .resolve(definitions, schema_store)
                                        .await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), Some(definitions))
                                        } else {
                                            (schema_url, Some(definitions))
                                        };

                                    return get_property_value_completion_contents(
                                        key,
                                        value,
                                        accessors,
                                        Some(additional_property_schema),
                                        toml_version,
                                        position,
                                        keys,
                                        schema_url,
                                        definitions,
                                        schema_store,
                                        completion_hint,
                                    )
                                    .await;
                                }
                            }

                            if table_schema.additional_properties {
                                return get_property_value_completion_contents(
                                    key,
                                    value,
                                    accessors,
                                    None,
                                    toml_version,
                                    position,
                                    keys,
                                    None,
                                    None,
                                    schema_store,
                                    completion_hint,
                                )
                                .await;
                            }
                        }
                    } else {
                        for (accessor, property) in table_schema.properties.write().await.iter_mut()
                        {
                            let key_name = &accessor.to_string();

                            if let Some(value) = self.get(key_name) {
                                if check_used_table_value(value) {
                                    continue;
                                }
                            }

                            // NOTE: To avoid downloading unnecessary schema files,
                            //       if the property is an unresolved online URL(like https:// or http://),
                            //       only the overview is used to generate completion candidates.
                            match property {
                                Referable::Ref {
                                    reference,
                                    title,
                                    description,
                                } if is_online_url(reference) => {
                                    completion_contents.push(CompletionContent::new_key(
                                        key_name,
                                        position,
                                        title.clone(),
                                        description.clone(),
                                        table_schema.required.as_ref(),
                                        schema_url,
                                        completion_hint,
                                    ));
                                    continue;
                                }
                                _ => {}
                            }

                            if let Ok((value_schema, new_schema)) =
                                property.resolve(definitions, schema_store).await
                            {
                                let (schema_url, definitions) =
                                    if let Some((schema_url, definitions)) = &new_schema {
                                        (Some(schema_url), definitions)
                                    } else {
                                        (schema_url, definitions)
                                    };

                                let Some(contents) = collect_table_key_completion_contents(
                                    self,
                                    table_schema,
                                    key_name,
                                    position,
                                    accessors,
                                    completion_hint,
                                    schema_url,
                                    value_schema,
                                    definitions,
                                    schema_store,
                                )
                                .await
                                else {
                                    continue;
                                };
                                completion_contents.extend(contents);
                            }
                        }

                        if completion_contents.is_empty() {
                            if let Some(pattern_properties) = &table_schema.pattern_properties {
                                let patterns = pattern_properties
                                    .read()
                                    .await
                                    .keys()
                                    .map(ToString::to_string)
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
                Some(ValueSchema::OneOf(one_of_schema)) => {
                    find_one_of_completion_items(
                        self,
                        accessors,
                        one_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                Some(ValueSchema::AnyOf(any_of_schema)) => {
                    find_any_of_completion_items(
                        self,
                        accessors,
                        any_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                Some(ValueSchema::AllOf(all_of_schema)) => {
                    find_all_of_completion_items(
                        self,
                        accessors,
                        all_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                Some(_) => Vec::with_capacity(0),
                None => {
                    if let Some(key) = keys.first() {
                        if let Some(value) = self.get(key) {
                            get_property_value_completion_contents(
                                key,
                                value,
                                accessors,
                                None,
                                toml_version,
                                position,
                                keys,
                                None,
                                None,
                                schema_store,
                                completion_hint,
                            )
                            .await
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
        .boxed()
    }
}

impl FindCompletionContents for TableSchema {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        async move {
            let Some(definitions) = definitions else {
                unreachable!("definitions must be provided");
            };

            let mut completion_items = Vec::new();

            for (key, property) in self.properties.write().await.iter_mut() {
                let label = &key.to_string();

                if let Ok((value_schema, new_schema)) =
                    property.resolve(definitions, schema_store).await
                {
                    let definitions = if let Some((_, definitions)) = &new_schema {
                        definitions
                    } else {
                        definitions
                    };
                    let (schema_candidates, errors) = value_schema
                        .find_schema_candidates(accessors, definitions, schema_store)
                        .await;

                    for error in errors {
                        tracing::error!("{}", error);
                    }

                    for schema_candidate in schema_candidates {
                        if let Some(CompletionHint::InTableHeader) = completion_hint {
                            if count_table_or_array_schema(value_schema, definitions, schema_store)
                                .await
                                == 0
                            {
                                continue;
                            }
                        }

                        completion_items.push(CompletionContent::new_key(
                            label,
                            position,
                            schema_candidate
                                .detail(definitions, schema_store, completion_hint)
                                .await,
                            schema_candidate
                                .documentation(definitions, schema_store, completion_hint)
                                .await,
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
        .boxed()
    }
}

async fn count_table_or_array_schema(
    value_schema: &ValueSchema,
    definitions: &SchemaDefinitions,
    schema_store: &SchemaStore,
) -> usize {
    join_all(
        value_schema
            .match_flattened_schemas(
                &|schema| matches!(schema, ValueSchema::Table(_) | ValueSchema::Array(_)),
                definitions,
                schema_store,
            )
            .await
            .into_iter()
            .map(|schema| async {
                match schema {
                    ValueSchema::Array(array_schema) => {
                        if let Some(item) = array_schema.items {
                            if let Ok((value_schema, new_schema)) =
                                item.write().await.resolve(definitions, schema_store).await
                            {
                                let definitions = if let Some((_, definitions)) = &new_schema {
                                    definitions
                                } else {
                                    definitions
                                };

                                return value_schema
                                    .is_match(
                                        &|schema| matches!(schema, ValueSchema::Table(_)),
                                        definitions,
                                        schema_store,
                                    )
                                    .await;
                            }
                        }
                        true
                    }
                    ValueSchema::Table(_) => true,
                    _ => unreachable!("only table and array are allowed"),
                }
            }),
    )
    .await
    .into_iter()
    .filter(|&is_table_or_array_schema| is_table_or_array_schema)
    .count()
}

fn get_property_value_completion_contents<'a: 'b, 'b>(
    key: &'a document_tree::Key,
    value: &'a document_tree::Value,
    accessors: &'a Vec<Accessor>,
    value_schema: Option<&'a ValueSchema>,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &'a [document_tree::Key],
    schema_url: Option<&'a SchemaUrl>,
    definitions: Option<&'a SchemaDefinitions>,
    schema_store: &'a SchemaStore,
    completion_hint: Option<CompletionHint>,
) -> BoxFuture<'b, Vec<CompletionContent>> {
    tracing::trace!("key: {:?}", key);
    tracing::trace!("value: {:?}", value);
    tracing::trace!("keys: {:?}", keys);
    tracing::trace!("accessors: {:?}", accessors);
    tracing::trace!("value schema: {:?}", value_schema);
    tracing::trace!("completion hint: {:?}", completion_hint);

    async move {
        if keys.len() == 1 {
            match completion_hint {
                Some(CompletionHint::InArray) => {
                    return type_hint_value(
                        Some(key),
                        position,
                        toml_version,
                        None,
                        completion_hint,
                    )
                }
                Some(
                    CompletionHint::DotTrigger { range } | CompletionHint::EqualTrigger { range },
                ) => {
                    let key = keys.first().unwrap();
                    if value_schema.is_none() {
                        if range.end() <= key.range().start() {
                            return vec![CompletionContent::new_type_hint_key(
                                key,
                                toml_version,
                                schema_url,
                                completion_hint,
                            )];
                        }
                        return type_hint_value(
                            Some(key),
                            position,
                            toml_version,
                            schema_url,
                            completion_hint,
                        );
                    }
                }
                Some(CompletionHint::InTableHeader) => {
                    if let (Some(value_schema), Some(definitions)) = (value_schema, definitions) {
                        if count_table_or_array_schema(value_schema, definitions, schema_store)
                            .await
                            == 0
                        {
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
                                    &key.to_raw_text(toml_version),
                                    position,
                                    schema_url,
                                );
                            }
                            Some(CompletionHint::InArray) => {
                                return vec![CompletionContent::new_type_hint_key(
                                    &key,
                                    toml_version,
                                    schema_url,
                                    completion_hint,
                                )];
                            }
                        }
                    }
                }
            }
        }

        value
            .find_completion_contents(
                &accessors
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(Accessor::Key(
                        key.to_raw_text(toml_version),
                    )))
                    .collect(),
                value_schema,
                toml_version,
                position,
                &keys[1..],
                schema_url,
                definitions,
                schema_store,
                completion_hint,
            )
            .await
    }
    .boxed()
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

fn collect_table_key_completion_contents<'a: 'b, 'b>(
    table: &'a document_tree::Table,
    table_schema: &'a TableSchema,
    key_name: &'a String,
    position: text::Position,
    accessors: &'a Vec<Accessor>,
    completion_hint: Option<CompletionHint>,
    schema_url: Option<&'a SchemaUrl>,
    value_schema: &'a ValueSchema,
    definitions: &'a SchemaDefinitions,
    schema_store: &'a SchemaStore,
) -> BoxFuture<'b, Option<Vec<CompletionContent>>> {
    async move {
        let mut completion_contents = Vec::new();

        let (schema_candidates, errors) = value_schema
            .find_schema_candidates(accessors, definitions, schema_store)
            .await;

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
                        && count_table_or_array_schema(value_schema, definitions, schema_store)
                            .await
                            == 0
                    {
                        return None;
                    }
                    if let ValueSchema::Table(table_schema) = value_schema {
                        if !table_schema.additional_properties
                            && !table_schema.has_additional_property_schema()
                            && table_schema.pattern_properties.is_none()
                            && table_schema.properties.read().await.keys().all(|key| {
                                let property_name = &key.to_string();
                                table.get(property_name).is_some()
                            })
                        {
                            return None;
                        }
                    }
                }
                ValueSchema::Null
                | ValueSchema::OneOf(_)
                | ValueSchema::AnyOf(_)
                | ValueSchema::AllOf(_) => {
                    unreachable!(
                        "Null, OneOf, AnyOf, and AllOf are not allowed in flattened schema"
                    );
                }
            }

            completion_contents.push(CompletionContent::new_key(
                key_name,
                position,
                schema_candidate
                    .detail(definitions, schema_store, completion_hint)
                    .await,
                schema_candidate
                    .documentation(definitions, schema_store, completion_hint)
                    .await,
                table_schema.required.as_ref(),
                schema_url,
                completion_hint,
            ));
        }

        Some(completion_contents)
    }
    .boxed()
}
