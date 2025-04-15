use std::borrow::Cow;

use futures::{future::BoxFuture, FutureExt};
use itertools::Itertools;
use schema_store::{Accessor, CurrentSchema, SchemaUrl};

use super::{GetTypeDefinition, TypeDefinition};

pub fn get_one_of_type_definition<'a: 'b, 'b, T>(
    value: &'a T,
    position: text::Position,
    keys: &'a [document_tree::Key],
    accessors: &'a [schema_store::Accessor],
    one_of_schema: &'a schema_store::OneOfSchema,
    schema_url: &'a SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a schema_store::SchemaContext,
) -> BoxFuture<'b, Option<TypeDefinition>>
where
    T: GetTypeDefinition + document_tree::ValueImpl + validator::Validate + Sync + Send,
{
    async move {
        for referable_schema in one_of_schema.schemas.write().await.iter_mut() {
            let Ok(Some(current_schema)) = referable_schema
                .resolve(
                    Cow::Borrowed(schema_url),
                    Cow::Borrowed(definitions),
                    schema_context.store,
                )
                .await
            else {
                continue;
            };

            if let Some(type_definition) = value
                .get_type_definition(
                    position,
                    keys,
                    accessors,
                    Some(&current_schema),
                    schema_context,
                )
                .await
            {
                if value
                    .validate(
                        &accessors
                            .iter()
                            .map(|accessor| accessor.into())
                            .collect_vec(),
                        Some(&current_schema),
                        schema_context,
                    )
                    .await
                    .is_ok()
                {
                    return Some(type_definition);
                }
            }
        }

        Some(TypeDefinition {
            schema_url: schema_url.clone(),
            range: text::Range::default(),
        })
    }
    .boxed()
}

impl GetTypeDefinition for schema_store::OneOfSchema {
    fn get_type_definition<'a: 'b, 'b>(
        &'a self,
        _position: text::Position,
        _keys: &'a [document_tree::Key],
        _accessors: &'a [Accessor],
        current_schema: Option<&'a CurrentSchema<'a>>,
        _schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Option<TypeDefinition>> {
        async move {
            let Some(current_schema) = current_schema else {
                unreachable!("schema must be provided");
            };

            Some(TypeDefinition {
                schema_url: current_schema.schema_url.as_ref().to_owned(),
                range: text::Range::default(),
            })
        }
        .boxed()
    }
}
