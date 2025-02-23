use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, Accessors, ArraySchema, SchemaAccessor, SchemaUrl, ValueSchema, ValueType,
};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, one_of::get_one_of_hover_content, GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Array {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        tracing::debug!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value_schema: {:?}", value_schema);

        async move {
            if let Some(sub_schema_url_map) = sub_schema_url_map {
                if let Some(sub_schema_url) = sub_schema_url_map.get(
                    &accessors
                        .iter()
                        .map(SchemaAccessor::from)
                        .collect::<Vec<_>>(),
                ) {
                    if schema_url != Some(sub_schema_url) {
                        if let Ok(document_schema) = schema_store
                            .try_get_document_schema_from_url(sub_schema_url)
                            .await
                        {
                            return self
                                .get_hover_content(
                                    accessors,
                                    document_schema.value_schema.as_ref(),
                                    toml_version,
                                    position,
                                    keys,
                                    Some(&document_schema.schema_url),
                                    &document_schema.definitions,
                                    Some(sub_schema_url_map),
                                    schema_store,
                                )
                                .await;
                        }
                    }
                }
            }

            match value_schema {
                Some(ValueSchema::Array(array_schema)) => {
                    for (index, value) in self.values().iter().enumerate() {
                        if value.range().contains(position) {
                            let accessor = Accessor::Index(index);

                            if let Some(items) = &array_schema.items {
                                let mut referable_schema = items.write().await;
                                if let Ok((item_schema, new_schema)) =
                                    referable_schema.resolve(definitions, schema_store).await
                                {
                                    let (schema_url, definitions) =
                                        if let Some((schema_url, definitions)) = &new_schema {
                                            (Some(schema_url), definitions)
                                        } else {
                                            (schema_url, definitions)
                                        };

                                    let mut hover_content = value
                                        .get_hover_content(
                                            &accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor.clone()))
                                                .collect::<Vec<_>>(),
                                            Some(item_schema),
                                            toml_version,
                                            position,
                                            keys,
                                            schema_url,
                                            definitions,
                                            sub_schema_url_map,
                                            schema_store,
                                        )
                                        .await?;

                                    if keys.is_empty()
                                        && self.kind() == document_tree::ArrayKind::ArrayOfTables
                                    {
                                        if let Some(constraints) = &mut hover_content.constraints {
                                            constraints.min_items = array_schema.min_items;
                                            constraints.max_items = array_schema.max_items;
                                            constraints.unique_items = array_schema.unique_items;
                                        }
                                    }

                                    if hover_content.title.is_none()
                                        && hover_content.description.is_none()
                                    {
                                        if let Some(title) = &array_schema.title {
                                            hover_content.title = Some(title.clone());
                                        }
                                        if let Some(description) = &array_schema.description {
                                            hover_content.description = Some(description.clone());
                                        }
                                    }
                                    return Some(hover_content);
                                }
                            }

                            return value
                                .get_hover_content(
                                    &accessors
                                        .iter()
                                        .cloned()
                                        .chain(std::iter::once(accessor))
                                        .collect::<Vec<_>>(),
                                    None,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    sub_schema_url_map,
                                    schema_store,
                                )
                                .await;
                        }
                    }
                    array_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                        .map(|mut hover_content| {
                            hover_content.range = Some(self.range());
                            hover_content
                        })
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
                        sub_schema_url_map,
                        schema_store,
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
                        sub_schema_url_map,
                        schema_store,
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
                        sub_schema_url_map,
                        schema_store,
                    )
                    .await
                }
                Some(_) => None,
                None => {
                    for (index, value) in self.values().iter().enumerate() {
                        if value.range().contains(position) {
                            let accessor = Accessor::Index(index);
                            return value
                                .get_hover_content(
                                    &accessors
                                        .iter()
                                        .cloned()
                                        .chain(std::iter::once(accessor))
                                        .collect::<Vec<_>>(),
                                    None,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    sub_schema_url_map,
                                    schema_store,
                                )
                                .await;
                        }
                    }
                    Some(HoverContent {
                        title: None,
                        description: None,
                        accessors: Accessors::new(accessors.to_vec()),
                        value_type: ValueType::Array,
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

impl GetHoverContent for ArraySchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        _definitions: &'a schema_store::SchemaDefinitions,
        _sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        _schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            Some(HoverContent {
                title: self.title.clone(),
                description: self.description.clone(),
                accessors: Accessors::new(accessors.to_vec()),
                value_type: ValueType::Array,
                constraints: Some(DataConstraints {
                    min_items: self.min_items,
                    max_items: self.max_items,
                    unique_items: self.unique_items,
                    ..Default::default()
                }),
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
