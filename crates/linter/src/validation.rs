mod all_of;
mod any_of;
mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod one_of;
mod string;
mod table;
mod value;

use all_of::validate_all_of;
use any_of::validate_any_of;
use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use one_of::validate_one_of;
use schema_store::{Accessor, SchemaDefinitions, SchemaUrl, ValueSchema};

pub trait Validate {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        sub_schema_url_map: &'a schema_store::SubSchemaUrlMap,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>;
}

pub fn validate<'a: 'b, 'b>(
    tree: document_tree::DocumentTree,
    toml_version: TomlVersion,
    source_schema: &'a schema_store::SourceSchema,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
    async move {
        tree.validate(
            toml_version,
            &Vec::with_capacity(0),
            source_schema
                .root_schema
                .as_ref()
                .and_then(|s| s.value_schema.as_ref()),
            source_schema.root_schema.as_ref().map(|s| &s.schema_url),
            source_schema.root_schema.as_ref().map(|s| &s.definitions),
            &source_schema.sub_schema_url_map,
            schema_store,
        )
        .await?;

        Ok(())
    }
    .boxed()
}
