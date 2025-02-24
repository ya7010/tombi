use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, LocalTimeSchema, SchemaUrl, ValueSchema};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, default_value::DefaultValue, one_of::get_one_of_hover_content,
    GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::LocalTime {
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
        async move {
            match value_schema {
                Some(ValueSchema::LocalTime(schema)) => schema
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
                    }),
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
                None => Some(HoverContent {
                    title: None,
                    description: None,
                    accessors: schema_store::Accessors::new(accessors.to_vec()),
                    value_type: schema_store::ValueType::LocalTime,
                    constraints: None,
                    schema_url: None,
                    range: Some(self.range()),
                }),
            }
        }
        .boxed()
    }
}

impl GetHoverContent for LocalTimeSchema {
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
                accessors: schema_store::Accessors::new(accessors.to_vec()),
                value_type: schema_store::ValueType::LocalTime,
                constraints: Some(DataConstraints {
                    default: self
                        .default
                        .as_ref()
                        .map(|value| DefaultValue::LocalTime(value.clone())),
                    enumerate: self.enumerate.as_ref().map(|value| {
                        value
                            .iter()
                            .map(|value| DefaultValue::LocalTime(value.clone()))
                            .collect()
                    }),
                    ..Default::default()
                }),
                schema_url: schema_url.cloned(),
                range: None,
            })
        }
        .boxed()
    }
}
