use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, Accessors, SchemaUrl, TableSchema, ValueSchema, ValueType};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, one_of::get_one_of_hover_content, GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Table {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        tracing::debug!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value_schema: {:?}", value_schema);

        async move {
            match value_schema {
                Some(ValueSchema::Table(table_schema)) => {
                    if let Some(key) = keys.first() {
                        if let Some(value) = self.get(key) {
                            let key_str = key.to_raw_text(toml_version);
                            let accessor = Accessor::Key(key_str.clone());
                            let key_patterns = table_schema.pattern_properties.as_ref().map(
                                |pattern_properties| {
                                    pattern_properties
                                        .iter()
                                        .map(|pattern_property| pattern_property.key().to_string())
                                        .collect::<Vec<_>>()
                                },
                            );

                            if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                                let required = table_schema
                                    .required
                                    .as_ref()
                                    .map(|r| r.contains(&key_str))
                                    .unwrap_or(false);

                                if let Ok((property_schema, new_schema)) =
                                    property.resolve(definitions, &schema_store).await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), definitions)
                                        } else {
                                            (schema_url, definitions)
                                        };
                                    return value
                                        .get_hover_content(
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
                                            &schema_store,
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
                                        &accessors
                                            .clone()
                                            .into_iter()
                                            .chain(std::iter::once(accessor))
                                            .collect(),
                                        None,
                                        toml_version,
                                        position,
                                        &keys[1..],
                                        schema_url,
                                        definitions,
                                        &schema_store,
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
                                for mut pattern_property in pattern_properties.iter_mut() {
                                    let property_key = pattern_property.key();
                                    if let Ok(pattern) = regex::Regex::new(property_key) {
                                        if pattern.is_match(&key_str) {
                                            if let Ok((property_schema, new_schema)) =
                                                pattern_property
                                                    .value_mut()
                                                    .resolve(definitions, &schema_store)
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
                                                        &schema_store,
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
                                                    &accessors
                                                        .clone()
                                                        .into_iter()
                                                        .chain(std::iter::once(accessor))
                                                        .collect(),
                                                    None,
                                                    toml_version,
                                                    position,
                                                    &keys[1..],
                                                    schema_url,
                                                    definitions,
                                                    &schema_store,
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
                                    referable_schema.resolve(definitions, &schema_store).await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), definitions)
                                        } else {
                                            (schema_url, definitions)
                                        };

                                    return value
                                        .get_hover_content(
                                            &accessors
                                                .clone()
                                                .into_iter()
                                                .chain(std::iter::once(accessor.clone()))
                                                .collect(),
                                            Some(additional_property_schema),
                                            toml_version,
                                            position,
                                            &keys[1..],
                                            schema_url,
                                            definitions,
                                            &schema_store,
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
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    None,
                                    toml_version,
                                    position,
                                    &keys[1..],
                                    schema_url,
                                    definitions,
                                    &schema_store,
                                )
                                .await
                        } else {
                            None
                        }
                    } else {
                        table_schema
                            .get_hover_content(
                                accessors,
                                value_schema,
                                toml_version,
                                position,
                                keys,
                                schema_url,
                                definitions,
                                &schema_store,
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
                        accessors,
                        one_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                Some(ValueSchema::AnyOf(any_of_schema)) => {
                    get_any_of_hover_content(
                        self,
                        accessors,
                        any_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                Some(ValueSchema::AllOf(all_of_schema)) => {
                    get_all_of_hover_content(
                        self,
                        accessors,
                        all_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                Some(_) => None,
                None => {
                    if let Some(key) = keys.first() {
                        if let Some(value) = self.get(key) {
                            let accessor = Accessor::Key(key.to_raw_text(toml_version));

                            return value
                                .get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    None,
                                    toml_version,
                                    position,
                                    &keys[1..],
                                    schema_url,
                                    definitions,
                                    &schema_store,
                                )
                                .await;
                        }
                    }
                    Some(HoverContent {
                        title: None,
                        description: None,
                        accessors: Accessors::new(accessors.clone()),
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
        accessors: &'a Vec<Accessor>,
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        _definitions: &'a schema_store::SchemaDefinitions,
        _schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            Some(HoverContent {
                title: self.title.clone(),
                description: self.description.clone(),
                accessors: Accessors::new(accessors.clone()),
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
