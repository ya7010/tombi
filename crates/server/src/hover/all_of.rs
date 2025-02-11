use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaUrl, ValueSchema};

use super::{GetHoverContent, HoverContent};

pub fn get_all_of_hover_content<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a Vec<schema_store::Accessor>,
    all_of_schema: &'a schema_store::AllOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &'a [document_tree::Key],
    schema_url: Option<&'a SchemaUrl>,
    definitions: &'a schema_store::SchemaDefinitions,
) -> BoxFuture<'b, Option<HoverContent>>
where
    T: GetHoverContent + Sync + Send,
{
    async move {
        let mut title_description_set = ahash::AHashSet::new();
        let mut value_type_set = indexmap::IndexSet::new();
        let mut schema = None;
        for referable_schema in all_of_schema.schemas.write().await.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions).await else {
                return None;
            };
            if let Some(hover_content) = value
                .get_hover_content(
                    accessors,
                    Some(value_schema),
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                )
                .await
            {
                if hover_content.title.is_some() || hover_content.description.is_some() {
                    title_description_set.insert((
                        hover_content.title.clone(),
                        hover_content.description.clone(),
                    ));
                    value_type_set.insert(hover_content.value_type);
                }

                if let Some(s) = hover_content.constraints {
                    schema = Some(s);
                }
            }
        }

        let (mut title, mut description) = if title_description_set.len() == 1 {
            title_description_set.into_iter().next().unwrap()
        } else {
            (None, None)
        };

        if title.is_none() && description.is_none() {
            if let Some(t) = &all_of_schema.title {
                title = Some(t.clone());
            }
            if let Some(d) = &all_of_schema.description {
                description = Some(d.clone());
            }
        }

        let value_type = if value_type_set.len() == 1 {
            value_type_set.into_iter().next().unwrap()
        } else {
            schema_store::ValueType::AllOf(value_type_set.into_iter().collect())
        };

        Some(HoverContent {
            title,
            description,
            accessors: schema_store::Accessors::new(accessors.clone()),
            value_type,
            constraints: schema,
            schema_url: schema_url.cloned(),
            range: None,
        })
    }
    .boxed()
}

impl GetHoverContent for schema_store::AllOfSchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            let mut title_description_set = ahash::AHashSet::new();
            let mut value_type_set = indexmap::IndexSet::new();
            let mut schemas = self.schemas.write().await;

            for referable_schema in schemas.iter_mut() {
                let Ok(value_schema) = referable_schema.resolve(definitions).await else {
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
                schema_store::ValueType::AllOf(value_type_set.into_iter().collect())
            };

            Some(HoverContent {
                title,
                description,
                accessors: schema_store::Accessors::new(accessors.clone()),
                value_type,
                constraints: None,
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
