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
use one_of::validate_one_of;

use config::TomlVersion;
use futures::future::BoxFuture;
use futures::FutureExt;
use schema_store::SchemaDefinitions;
use schema_store::ValueSchema;
use std::ops::Deref;

trait Validate {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>;
}

pub fn validate<'a: 'b, 'b>(
    tree: document_tree::DocumentTree,
    toml_version: TomlVersion,
    schema_schema: &'a schema_store::SourceSchema,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
    async move {
        let table = tree.deref();
        let Some(document_schema) = schema_schema.root.as_ref() else {
            return Ok(());
        };

        if let Some(value_schema) = &document_schema.value_schema {
            table
                .validate(
                    toml_version,
                    value_schema,
                    &document_schema.definitions,
                    schema_store,
                )
                .await?;
        }

        Ok(())
    }
    .boxed()
}
