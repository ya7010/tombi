use std::borrow::Cow;

use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, CurrentSchema, DocumentSchema, SchemaAccessor, TableSchema, ValueSchema,
};

use crate::goto_type_definition::{
    all_of::get_all_of_type_definition, any_of::get_any_of_type_definition,
    one_of::get_one_of_type_definition, GetTypeDefinition, TypeDefinition,
};

impl GetTypeDefinition for tombi_document_tree::Table {
    fn get_type_definition<'a: 'b, 'b>(
        &'a self,
        position: text::Position,
        keys: &'a [tombi_document_tree::Key],
        accessors: &'a [Accessor],
        current_schema: Option<&'a CurrentSchema<'a>>,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<TypeDefinition>> {
        tracing::trace!("self = {:?}", self);
        tracing::trace!("keys = {:?}", keys);
        tracing::trace!("accessors = {:?}", accessors);
        tracing::trace!("current_schema = {:?}", current_schema);

        async move {
            if let Some(Ok(DocumentSchema {
                value_schema,
                schema_url,
                definitions,
                ..
            })) = schema_context
                .get_subschema(accessors, current_schema)
                .await
            {
                let current_schema = value_schema.map(|value_schema| CurrentSchema {
                    value_schema: Cow::Owned(value_schema),
                    schema_url: Cow::Owned(schema_url),
                    definitions: Cow::Owned(definitions),
                });

                return self
                    .get_type_definition(
                        position,
                        keys,
                        accessors,
                        current_schema.as_ref(),
                        schema_context,
                    )
                    .await;
            }

            if let Some(current_schema) = current_schema {
                match current_schema.value_schema.as_ref() {
                    ValueSchema::Table(table_schema) => {
                        if let Some(key) = keys.first() {
                            if let Some(value) = self.get(key) {
                                let key_str = key.to_raw_text(schema_context.toml_version);
                                let accessor = Accessor::Key(key_str.clone());

                                if let Some(property_schema) = table_schema
                                    .properties
                                    .write()
                                    .await
                                    .get_mut(&SchemaAccessor::from(&accessor))
                                {
                                    tracing::trace!("property_schema = {:?}", property_schema);

                                    if let Ok(Some(current_schema)) = property_schema
                                        .resolve(
                                            current_schema.schema_url.clone(),
                                            current_schema.definitions.clone(),
                                            schema_context.store,
                                        )
                                        .await
                                    {
                                        return value
                                            .get_type_definition(
                                                position,
                                                &keys[1..],
                                                &accessors
                                                    .iter()
                                                    .cloned()
                                                    .chain(std::iter::once(accessor))
                                                    .collect::<Vec<_>>(),
                                                Some(&current_schema),
                                                schema_context,
                                            )
                                            .await;
                                    }

                                    return value
                                        .get_type_definition(
                                            position,
                                            &keys[1..],
                                            &accessors
                                                .iter()
                                                .cloned()
                                                .chain(std::iter::once(accessor))
                                                .collect::<Vec<_>>(),
                                            None,
                                            schema_context,
                                        )
                                        .await;
                                }
                                if let Some(pattern_properties) = &table_schema.pattern_properties {
                                    for (property_key, pattern_property) in
                                        pattern_properties.write().await.iter_mut()
                                    {
                                        if let Ok(pattern) = regex::Regex::new(property_key) {
                                            if pattern.is_match(&key_str) {
                                                if let Ok(Some(current_schema)) = pattern_property
                                                    .resolve(
                                                        current_schema.schema_url.clone(),
                                                        current_schema.definitions.clone(),
                                                        schema_context.store,
                                                    )
                                                    .await
                                                {
                                                    return value
                                                        .get_type_definition(
                                                            position,
                                                            &keys[1..],
                                                            &accessors
                                                                .iter()
                                                                .cloned()
                                                                .chain(std::iter::once(accessor))
                                                                .collect::<Vec<_>>(),
                                                            Some(&current_schema),
                                                            schema_context,
                                                        )
                                                        .await;
                                                }

                                                return value
                                                    .get_type_definition(
                                                        position,
                                                        &keys[1..],
                                                        &accessors
                                                            .iter()
                                                            .cloned()
                                                            .chain(std::iter::once(accessor))
                                                            .collect::<Vec<_>>(),
                                                        None,
                                                        schema_context,
                                                    )
                                                    .await;
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
                                    if let Ok(Some(current_schema)) = referable_schema
                                        .resolve(
                                            current_schema.schema_url.clone(),
                                            current_schema.definitions.clone(),
                                            schema_context.store,
                                        )
                                        .await
                                    {
                                        return value
                                            .get_type_definition(
                                                position,
                                                &keys[1..],
                                                &accessors
                                                    .iter()
                                                    .cloned()
                                                    .chain(std::iter::once(accessor.clone()))
                                                    .collect::<Vec<_>>(),
                                                Some(&current_schema),
                                                schema_context,
                                            )
                                            .await;
                                    }
                                }

                                value
                                    .get_type_definition(
                                        position,
                                        &keys[1..],
                                        &accessors
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(accessor))
                                            .collect::<Vec<_>>(),
                                        None,
                                        schema_context,
                                    )
                                    .await
                            } else {
                                Some(TypeDefinition {
                                    schema_url: current_schema.schema_url.as_ref().clone(),
                                    range: text::Range::default(),
                                })
                            }
                        } else {
                            table_schema
                                .get_type_definition(
                                    position,
                                    keys,
                                    accessors,
                                    Some(current_schema),
                                    schema_context,
                                )
                                .await
                        }
                    }
                    ValueSchema::OneOf(one_of_schema) => {
                        get_one_of_type_definition(
                            self,
                            position,
                            keys,
                            accessors,
                            one_of_schema,
                            &current_schema.schema_url,
                            &current_schema.definitions,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AnyOf(any_of_schema) => {
                        get_any_of_type_definition(
                            self,
                            position,
                            keys,
                            accessors,
                            any_of_schema,
                            &current_schema.schema_url,
                            &current_schema.definitions,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AllOf(all_of_schema) => {
                        get_all_of_type_definition(
                            self,
                            position,
                            keys,
                            accessors,
                            all_of_schema,
                            &current_schema.schema_url,
                            &current_schema.definitions,
                            schema_context,
                        )
                        .await
                    }
                    _ => Some(TypeDefinition {
                        schema_url: current_schema.schema_url.as_ref().clone(),
                        range: text::Range::default(),
                    }),
                }
            } else {
                if let Some(key) = keys.first() {
                    if let Some(value) = self.get(key) {
                        let accessor = Accessor::Key(key.to_raw_text(schema_context.toml_version));

                        return value
                            .get_type_definition(
                                position,
                                &keys[1..],
                                &accessors
                                    .iter()
                                    .cloned()
                                    .chain(std::iter::once(accessor))
                                    .collect::<Vec<_>>(),
                                None,
                                schema_context,
                            )
                            .await;
                    }
                }
                None
            }
        }
        .boxed()
    }
}

impl GetTypeDefinition for TableSchema {
    fn get_type_definition<'a: 'b, 'b>(
        &'a self,
        _position: text::Position,
        _keys: &'a [tombi_document_tree::Key],
        _accessors: &'a [Accessor],
        current_schema: Option<&'a CurrentSchema<'a>>,
        _schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<TypeDefinition>> {
        async move {
            current_schema.map(|schema| TypeDefinition {
                schema_url: schema.schema_url.as_ref().clone(),
                range: text::Range::default(),
            })
        }
        .boxed()
    }
}
