use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaUrl, ValueSchema};

use super::{GetHoverContent, HoverContent};

pub fn get_any_of_hover_content<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a [schema_store::Accessor],
    any_of_schema: &'a schema_store::AnyOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &'a [document_tree::Key],
    schema_url: Option<&'a SchemaUrl>,
    definitions: &'a schema_store::SchemaDefinitions,
    sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Option<HoverContent>>
where
    T: GetHoverContent + document_tree::ValueImpl + Sync + Send,
{
    async move {
        let mut value_type_set = indexmap::IndexSet::new();

        for referable_schema in any_of_schema.schemas.write().await.iter_mut() {
            let Ok((value_schema, _)) = referable_schema.resolve(definitions, schema_store).await
            else {
                continue;
            };
            value_type_set.insert(value_schema.value_type().await);
        }

        let value_type = if value_type_set.len() == 1 {
            value_type_set.into_iter().next().unwrap()
        } else {
            schema_store::ValueType::AnyOf(value_type_set.into_iter().collect())
        };

        for referable_schema in any_of_schema.schemas.read().await.iter() {
            let Some(value_schema) = referable_schema.resolved() else {
                continue;
            };

            if let Some(mut hover_content) = value
                .get_hover_content(
                    accessors,
                    Some(value_schema),
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    sub_schema_url_map,
                    schema_store,
                )
                .await
            {
                if hover_content.title.is_none() && hover_content.description.is_none() {
                    if let Some(title) = &any_of_schema.title {
                        hover_content.title = Some(title.clone());
                    }
                    if let Some(description) = &any_of_schema.description {
                        hover_content.description = Some(description.clone());
                    }
                }

                if keys.is_empty() && accessors == hover_content.accessors.as_ref() {
                    hover_content.value_type = value_type;
                }

                return Some(hover_content);
            }
        }

        Some(HoverContent {
            title: None,
            description: None,
            accessors: schema_store::Accessors::new(accessors.to_vec()),
            value_type: value.value_type().into(),
            constraints: None,
            schema_url: schema_url.cloned(),
            range: None,
        })
    }
    .boxed()
}

impl GetHoverContent for schema_store::AnyOfSchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        _sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            let mut title_description_set = ahash::AHashSet::new();
            let mut value_type_set = indexmap::IndexSet::new();

            for referable_schema in self.schemas.write().await.iter_mut() {
                let Ok((value_schema, _)) =
                    referable_schema.resolve(definitions, schema_store).await
                else {
                    return None;
                };
                if value_schema.title().is_some() || value_schema.description().is_some() {
                    title_description_set.insert((
                        value_schema.title().map(ToString::to_string),
                        value_schema.description().map(ToString::to_string),
                    ));
                }
                value_type_set.insert(value_schema.value_type().await);
            }

            let (mut title, mut description) = if title_description_set.len() == 1 {
                title_description_set.into_iter().next().unwrap()
            } else {
                (None, None)
            };

            if title.is_none() && description.is_none() {
                if let Some(t) = &self.title {
                    title = Some(t.clone());
                }
                if let Some(d) = &self.description {
                    description = Some(d.clone());
                }
            }

            let value_type: schema_store::ValueType = if value_type_set.len() == 1 {
                value_type_set.into_iter().next().unwrap()
            } else {
                schema_store::ValueType::AnyOf(value_type_set.into_iter().collect())
            };

            Some(HoverContent {
                title,
                description,
                accessors: schema_store::Accessors::new(accessors.to_vec()),
                value_type,
                constraints: None,
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
