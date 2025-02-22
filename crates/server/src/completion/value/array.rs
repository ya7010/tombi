use crate::completion::{
    completion_kind::CompletionKind, schema_completion::SchemaCompletion, CompletionContent,
    CompletionEdit,
};

use super::{
    all_of::find_all_of_completion_items, any_of::find_any_of_completion_items,
    one_of::find_one_of_completion_items, type_hint_value, CompletionHint, FindCompletionContents,
};
use config::TomlVersion;
use document_tree::ArrayKind;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, ArraySchema, SchemaAccessor, SchemaDefinitions, SchemaStore, SchemaUrl, ValueSchema,
};

impl FindCompletionContents for document_tree::Array {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        tracing::trace!("self: {:?}", self);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("value schema: {:?}", value_schema);
        tracing::trace!("completion hint: {:?}", completion_hint);

        async move {
            if let Some(sub_schema_url_map) = sub_schema_url_map {
                if let Some(sub_schema_url) = sub_schema_url_map.get(
                    &accessors
                        .into_iter()
                        .map(|accessor| SchemaAccessor::from(accessor))
                        .collect::<Vec<_>>(),
                ) {
                    if schema_url != Some(sub_schema_url) {
                        if let Ok(document_schema) = schema_store
                            .try_get_document_schema_from_url(&sub_schema_url)
                            .await
                        {
                            return self
                                .find_completion_contents(
                                    accessors,
                                    document_schema.value_schema.as_ref(),
                                    toml_version,
                                    position,
                                    keys,
                                    Some(&document_schema.schema_url),
                                    Some(&document_schema.definitions),
                                    Some(sub_schema_url_map),
                                    schema_store,
                                    completion_hint,
                                )
                                .await;
                        }
                    }
                }
            }

            match value_schema {
                Some(ValueSchema::Array(array_schema)) => {
                    let Some(definitions) = definitions else {
                        unreachable!("definitions must be provided");
                    };

                    let mut new_item_index = 0;
                    for (index, value) in self.values().iter().enumerate() {
                        if value.range().end() < position {
                            new_item_index = index + 1;
                        }
                        if value.range().contains(position) || value.range().end() == position {
                            let accessor = Accessor::Index(index);
                            if let Some(items) = &array_schema.items {
                                if let Ok((item_schema, new_schema)) =
                                    items.write().await.resolve(definitions, schema_store).await
                                {
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
                                            Some(item_schema),
                                            toml_version,
                                            position,
                                            keys,
                                            schema_url,
                                            definitions,
                                            sub_schema_url_map,
                                            schema_store,
                                            completion_hint,
                                        )
                                        .await;
                                }
                            }
                        }
                    }
                    if let Some(items) = &array_schema.items {
                        if let Ok((item_schema, new_schema)) =
                            items.write().await.resolve(definitions, schema_store).await
                        {
                            let (schema_url, definitions) =
                                if let Some((schema_url, definitions)) = &new_schema {
                                    (Some(schema_url), Some(definitions))
                                } else {
                                    (schema_url, Some(definitions))
                                };
                            return SchemaCompletion
                                .find_completion_contents(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(Accessor::Index(new_item_index)))
                                        .collect(),
                                    Some(item_schema),
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    sub_schema_url_map,
                                    schema_store,
                                    if self.kind() == ArrayKind::Array {
                                        Some(CompletionHint::InArray)
                                    } else {
                                        completion_hint
                                    },
                                )
                                .await;
                        }
                    }

                    Vec::with_capacity(0)
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
                        sub_schema_url_map,
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
                        sub_schema_url_map,
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
                        sub_schema_url_map,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                Some(_) => Vec::with_capacity(0),
                None => {
                    for (index, value) in self.values().iter().enumerate() {
                        if value.range().contains(position) || value.range().end() == position {
                            if let document_tree::Value::Table(table) = value {
                                if keys.len() == 1
                                    && table.kind() == document_tree::TableKind::KeyValue
                                {
                                    let key = &keys.first().unwrap();
                                    return vec![CompletionContent::new_type_hint_key(
                                        key,
                                        toml_version,
                                        schema_url,
                                        Some(CompletionHint::InArray),
                                    )];
                                }
                            }

                            let accessor = Accessor::Index(index);
                            return value
                                .find_completion_contents(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    None,
                                    toml_version,
                                    position,
                                    keys,
                                    None,
                                    None,
                                    sub_schema_url_map,
                                    schema_store,
                                    completion_hint,
                                )
                                .await;
                        }
                    }
                    type_hint_value(None, position, toml_version, schema_url, completion_hint)
                }
            }
        }
        .boxed()
    }
}

impl FindCompletionContents for ArraySchema {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a Vec<Accessor>,
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        _definitions: Option<&'a SchemaDefinitions>,
        _sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        _schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        async move {
            match completion_hint {
                Some(CompletionHint::InTableHeader) => Vec::with_capacity(0),
                _ => type_hint_array(position, schema_url, completion_hint),
            }
        }
        .boxed()
    }
}

pub fn type_hint_array(
    position: text::Position,
    schema_url: Option<&SchemaUrl>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    let edit = CompletionEdit::new_array_literal(position, completion_hint);

    vec![CompletionContent::new_type_hint_value(
        CompletionKind::Array,
        "[]",
        "Array",
        edit,
        schema_url,
    )]
}
