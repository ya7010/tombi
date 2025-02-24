use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, Accessors, SchemaAccessor, SchemaUrl, TableSchema, ValueSchema, ValueType,
};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, one_of::get_one_of_hover_content, GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Table {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        position: text::Position,
        keys: &'a [document_tree::Key],
        accessors: &'a [Accessor],
        schema_url: Option<&'a SchemaUrl>,
        value_schema: Option<&'a ValueSchema>,
        definitions: &'a schema_store::SchemaDefinitions,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        tracing::debug!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value_schema: {:?}", value_schema);

        async move {
            if let Some(sub_schema_url_map) = schema_context.sub_schema_url_map {
                if let Some(sub_schema_url) = sub_schema_url_map.get(
                    &accessors
                        .iter()
                        .map(SchemaAccessor::from)
                        .collect::<Vec<_>>(),
                ) {
                    if schema_url != Some(sub_schema_url) {
                        if let Ok(document_schema) = schema_context
                            .store
                            .try_get_document_schema_from_url(sub_schema_url)
                            .await
                        {
                            return self
                                .get_hover_content(
                                    position,
                                    keys,
                                    accessors,
                                    Some(&document_schema.schema_url),
                                    document_schema.value_schema.as_ref(),
                                    &document_schema.definitions,
                                    schema_context,
                                )
                                .await;
                        }
                    }
                }
            }

            match value_schema {
                Some(ValueSchema::Table(table_schema)) => {
                    if let Some(key) = keys.first() {
                        if let Some(value) = self.get(key) {
                            let key_str = key.to_raw_text(schema_context.toml_version);
                            let accessor = Accessor::Key(key_str.clone());
                            let key_patterns = match table_schema.pattern_properties.as_ref() {
                                Some(pattern_properties) => Some(
                                    pattern_properties
                                        .read()
                                        .await
                                        .keys()
                                        .map(ToString::to_string)
                                        .collect::<Vec<_>>(),
                                ),
                                None => None,
                            };

                            if let Some(property) =
                                table_schema.properties.write().await.get_mut(&accessor)
                            {
                                let required = table_schema
                                    .required
                                    .as_ref()
                                    .map(|r| r.contains(&key_str))
                                    .unwrap_or(false);

                                if let Ok((property_schema, new_schema)) =
                                    property.resolve(definitions, schema_context.store).await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), definitions)
                                        } else {
                                            (schema_url, definitions)
                                        };
                                    return value
                                        .get_hover_content(
                                            position,
                                            &keys[1..],
                                            &accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor))
                                                .collect::<Vec<_>>(),
                                            schema_url,
                                            Some(property_schema),
                                            definitions,
                                            schema_context,
                                        )
                                        .await
                                        .map(|mut hover_content| {
                                            if keys.len() == 1
                                                && !required
                                                && hover_content
                                                    .accessors
                                                    .last()
                                                    .map(|accessor| accessor.is_key())
                                                    .unwrap_or_default()
                                            {
                                                if let Some(constraints) =
                                                    &mut hover_content.constraints
                                                {
                                                    constraints.key_patterns = key_patterns;
                                                }
                                                hover_content.into_nullable()
                                            } else {
                                                hover_content
                                            }
                                        });
                                }

                                return value
                                    .get_hover_content(
                                        position,
                                        &keys[1..],
                                        &accessors
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(accessor))
                                            .collect::<Vec<_>>(),
                                        schema_url,
                                        None,
                                        definitions,
                                        schema_context,
                                    )
                                    .await
                                    .map(|mut hover_content| {
                                        if keys.len() == 1
                                            && !required
                                            && hover_content
                                                .accessors
                                                .last()
                                                .map(|accessor| accessor.is_key())
                                                .unwrap_or_default()
                                        {
                                            if let Some(constraints) =
                                                &mut hover_content.constraints
                                            {
                                                constraints.key_patterns = key_patterns;
                                            }
                                            hover_content.into_nullable()
                                        } else {
                                            hover_content
                                        }
                                    });
                            }
                            if let Some(pattern_properties) = &table_schema.pattern_properties {
                                for (property_key, pattern_property) in
                                    pattern_properties.write().await.iter_mut()
                                {
                                    if let Ok(pattern) = regex::Regex::new(property_key) {
                                        if pattern.is_match(&key_str) {
                                            if let Ok((property_schema, new_schema)) =
                                                pattern_property
                                                    .resolve(definitions, schema_context.store)
                                                    .await
                                            {
                                                let (schema_url, definitions) =
                                                    if let Some((schema_url, definitions)) =
                                                        &new_schema
                                                    {
                                                        (Some(schema_url), definitions)
                                                    } else {
                                                        (schema_url, definitions)
                                                    };
                                                return value
                                                    .get_hover_content(
                                                        position,
                                                        &keys[1..],
                                                        &accessors
                                                            .iter()
                                                            .cloned()
                                                            .chain(std::iter::once(accessor))
                                                            .collect::<Vec<_>>(),
                                                        schema_url,
                                                        Some(property_schema),
                                                        definitions,
                                                        schema_context,
                                                    )
                                                    .await
                                                    .map(|mut hover_content| {
                                                        if keys.len() == 1
                                                            && hover_content
                                                                .accessors
                                                                .last()
                                                                .map(|accessor| accessor.is_key())
                                                                .unwrap_or_default()
                                                        {
                                                            if let Some(constraints) =
                                                                &mut hover_content.constraints
                                                            {
                                                                constraints.key_patterns =
                                                                    key_patterns;
                                                            }
                                                            hover_content.into_nullable()
                                                        } else {
                                                            hover_content
                                                        }
                                                    });
                                            }

                                            return value
                                                .get_hover_content(
                                                    position,
                                                    &keys[1..],
                                                    &accessors
                                                        .iter()
                                                        .cloned()
                                                        .chain(std::iter::once(accessor))
                                                        .collect::<Vec<_>>(),
                                                    schema_url,
                                                    None,
                                                    definitions,
                                                    schema_context,
                                                )
                                                .await
                                                .map(|mut hover_content| {
                                                    if keys.len() == 1
                                                        && hover_content
                                                            .accessors
                                                            .last()
                                                            .map(|accessor| accessor.is_key())
                                                            .unwrap_or_default()
                                                    {
                                                        if let Some(constraints) =
                                                            &mut hover_content.constraints
                                                        {
                                                            constraints.key_patterns = key_patterns;
                                                        }
                                                        hover_content.into_nullable()
                                                    } else {
                                                        hover_content
                                                    }
                                                });
                                        }
                                    } else {
                                        tracing::error!(
                                            "Invalid regex pattern property: {}",
                                            property_key
                                        );
                                    };
                                }
                            }

                            if let Some(referable_additional_property_schema) =
                                &table_schema.additional_property_schema
                            {
                                let mut referable_schema =
                                    referable_additional_property_schema.write().await;
                                if let Ok((additional_property_schema, new_schema)) =
                                    referable_schema
                                        .resolve(definitions, schema_context.store)
                                        .await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), definitions)
                                        } else {
                                            (schema_url, definitions)
                                        };

                                    return value
                                        .get_hover_content(
                                            position,
                                            &keys[1..],
                                            &accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor.clone()))
                                                .collect::<Vec<_>>(),
                                            schema_url,
                                            Some(additional_property_schema),
                                            definitions,
                                            schema_context,
                                        )
                                        .await
                                        .map(|hover_content| {
                                            if keys.len() == 1
                                                && hover_content
                                                    .accessors
                                                    .last()
                                                    .map(|accessor| accessor.is_key())
                                                    .unwrap_or_default()
                                            {
                                                hover_content.into_nullable()
                                            } else {
                                                hover_content
                                            }
                                        });
                                }
                            }

                            value
                                .get_hover_content(
                                    position,
                                    &keys[1..],
                                    &accessors
                                        .iter()
                                        .cloned()
                                        .chain(std::iter::once(accessor))
                                        .collect::<Vec<_>>(),
                                    schema_url,
                                    None,
                                    definitions,
                                    schema_context,
                                )
                                .await
                        } else {
                            None
                        }
                    } else {
                        table_schema
                            .get_hover_content(
                                position,
                                keys,
                                accessors,
                                schema_url,
                                value_schema,
                                definitions,
                                schema_context,
                            )
                            .await
                            .map(|mut hover_content| {
                                hover_content.range = Some(self.range());
                                hover_content
                            })
                    }
                }
                Some(ValueSchema::OneOf(one_of_schema)) => {
                    get_one_of_hover_content(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        one_of_schema,
                        definitions,
                        schema_context,
                    )
                    .await
                }
                Some(ValueSchema::AnyOf(any_of_schema)) => {
                    get_any_of_hover_content(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        any_of_schema,
                        definitions,
                        schema_context,
                    )
                    .await
                }
                Some(ValueSchema::AllOf(all_of_schema)) => {
                    get_all_of_hover_content(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        all_of_schema,
                        definitions,
                        schema_context,
                    )
                    .await
                }
                Some(_) => None,
                None => {
                    if let Some(key) = keys.first() {
                        if let Some(value) = self.get(key) {
                            let accessor =
                                Accessor::Key(key.to_raw_text(schema_context.toml_version));

                            return value
                                .get_hover_content(
                                    position,
                                    &keys[1..],
                                    &accessors
                                        .iter()
                                        .cloned()
                                        .chain(std::iter::once(accessor))
                                        .collect::<Vec<_>>(),
                                    schema_url,
                                    None,
                                    definitions,
                                    schema_context,
                                )
                                .await;
                        }
                    }
                    Some(HoverContent {
                        title: None,
                        description: None,
                        accessors: Accessors::new(accessors.to_vec()),
                        value_type: ValueType::Table,
                        constraints: None,
                        schema_url: None,
                        range: Some(self.range()),
                    })
                }
            }
        }
        .boxed()
    }
}

impl GetHoverContent for TableSchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        accessors: &'a [Accessor],
        schema_url: Option<&'a SchemaUrl>,
        _value_schema: Option<&'a ValueSchema>,
        _definitions: &'a schema_store::SchemaDefinitions,
        _schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            Some(HoverContent {
                title: self.title.clone(),
                description: self.description.clone(),
                accessors: Accessors::new(accessors.to_vec()),
                value_type: ValueType::Table,
                constraints: Some(DataConstraints {
                    required_keys: self.required.clone(),
                    max_keys: self.max_properties,
                    min_keys: self.min_properties,
                    // NOTE: key_patterns are output for keys, not this tables.
                    key_patterns: None,
                    additional_keys: Some(self.additional_properties),
                    ..Default::default()
                }),
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
