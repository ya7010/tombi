use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, OffsetDateTimeSchema, SchemaUrl, ValueSchema};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, default_value::DefaultValue, one_of::get_one_of_hover_content,
    GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::OffsetDateTime {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            match value_schema {
                Some(ValueSchema::OffsetDateTime(offset_date_time_schema)) => {
                    offset_date_time_schema
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
                None => Some(HoverContent {
                    title: None,
                    description: None,
                    accessors: schema_store::Accessors::new(accessors.clone()),
                    value_type: schema_store::ValueType::OffsetDateTime,
                    constraints: None,
                    schema_url: None,
                    range: Some(self.range()),
                }),
            }
        }
        .boxed()
    }
}

impl GetHoverContent for OffsetDateTimeSchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
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
                accessors: schema_store::Accessors::new(accessors.clone()),
                value_type: schema_store::ValueType::OffsetDateTime,
                constraints: Some(DataConstraints {
                    default: self
                        .default
                        .as_ref()
                        .map(|value| DefaultValue::OffsetDateTime(value.clone())),
                    enumerate: self.enumerate.as_ref().map(|value| {
                        value
                            .iter()
                            .map(|value| DefaultValue::OffsetDateTime(value.clone()))
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
