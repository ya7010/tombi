use std::borrow::Cow;

use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, Accessors, ArraySchema, CurrentSchema, SchemaAccessor, SchemaUrl, ValueSchema,
    ValueType,
};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, one_of::get_one_of_hover_content, GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Array {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        position: text::Position,
        keys: &'a [document_tree::Key],
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        tracing::trace!("self: {:?}", self);
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
                        if let Ok(Some(document_schema)) = schema_context
                            .store
                            .try_get_document_schema(sub_schema_url)
                            .await
                        {
                            return self
                                .get_hover_content(
                                    position,
                                    keys,
                                    accessors,
                                    document_schema.value_schema.as_ref(),
                                    Some(&document_schema.schema_url),
                                    Some(&document_schema.definitions),
                                    schema_context,
                                )
                                .await;
                        }
                    }
                }
            }

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                match value_schema {
                    ValueSchema::Array(array_schema) => {
                        for (index, value) in self.values().iter().enumerate() {
                            if value.range().contains(position) {
                                let accessor = Accessor::Index(index);

                                if let Some(items) = &array_schema.items {
                                    let mut referable_schema = items.write().await;
                                    if let Ok(Some(CurrentSchema {
                                        schema_url,
                                        value_schema: item_schema,
                                        definitions,
                                    })) = referable_schema
                                        .resolve(
                                            Cow::Borrowed(schema_url),
                                            Cow::Borrowed(definitions),
                                            schema_context.store,
                                        )
                                        .await
                                    {
                                        let mut hover_content = value
                                            .get_hover_content(
                                                position,
                                                keys,
                                                &accessors
                                                    .iter()
                                                    .cloned()
                                                    .chain(std::iter::once(accessor.clone()))
                                                    .collect::<Vec<_>>(),
                                                Some(item_schema),
                                                Some(&schema_url),
                                                Some(&definitions),
                                                schema_context,
                                            )
                                            .await?;

                                        if keys.is_empty()
                                            && self.kind()
                                                == document_tree::ArrayKind::ArrayOfTables
                                        {
                                            if let Some(constraints) =
                                                &mut hover_content.constraints
                                            {
                                                constraints.min_items = array_schema.min_items;
                                                constraints.max_items = array_schema.max_items;
                                                constraints.unique_items =
                                                    array_schema.unique_items;
                                            }
                                        }

                                        if hover_content.title.is_none()
                                            && hover_content.description.is_none()
                                        {
                                            if let Some(title) = &array_schema.title {
                                                hover_content.title = Some(title.clone());
                                            }
                                            if let Some(description) = &array_schema.description {
                                                hover_content.description =
                                                    Some(description.clone());
                                            }
                                        }
                                        return Some(hover_content);
                                    }
                                }

                                return value
                                    .get_hover_content(
                                        position,
                                        keys,
                                        &accessors
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(accessor))
                                            .collect::<Vec<_>>(),
                                        Some(value_schema),
                                        Some(schema_url),
                                        Some(definitions),
                                        schema_context,
                                    )
                                    .await;
                            }
                        }
                        array_schema
                            .get_hover_content(
                                position,
                                keys,
                                accessors,
                                Some(value_schema),
                                Some(schema_url),
                                Some(definitions),
                                schema_context,
                            )
                            .await
                            .map(|mut hover_content| {
                                hover_content.range = Some(self.range());
                                hover_content
                            })
                    }
                    ValueSchema::OneOf(one_of_schema) => {
                        get_one_of_hover_content(
                            self,
                            position,
                            keys,
                            accessors,
                            one_of_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AnyOf(any_of_schema) => {
                        get_any_of_hover_content(
                            self,
                            position,
                            keys,
                            accessors,
                            any_of_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AllOf(all_of_schema) => {
                        get_all_of_hover_content(
                            self,
                            position,
                            keys,
                            accessors,
                            all_of_schema,
                            schema_url,
                            definitions,
                            schema_context,
                        )
                        .await
                    }
                    _ => None,
                }
            } else {
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);
                        return value
                            .get_hover_content(
                                position,
                                keys,
                                &accessors
                                    .iter()
                                    .cloned()
                                    .chain(std::iter::once(accessor))
                                    .collect::<Vec<_>>(),
                                value_schema,
                                schema_url,
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
                    value_type: ValueType::Array,
                    constraints: None,
                    schema_url: None,
                    range: Some(self.range()),
                })
            }
        }
        .boxed()
    }
}

impl GetHoverContent for ArraySchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        accessors: &'a [Accessor],
        _value_schema: Option<&'a ValueSchema>,
        schema_url: Option<&'a SchemaUrl>,
        _definitions: Option<&'a schema_store::SchemaDefinitions>,
        _schema_context: &'a schema_store::SchemaContext,
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
                    values_order: self.values_order.clone(),
                    ..Default::default()
                }),
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
