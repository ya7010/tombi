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
use futures::{future::BoxFuture, FutureExt};
use one_of::validate_one_of;

pub trait Validate {
    fn validate<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>>;
}

pub fn validate<'a: 'b, 'b>(
    tree: document_tree::DocumentTree,
    source_schema: &'a schema_store::SourceSchema,
    schema_context: &'a schema_store::SchemaContext,
) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>> {
    async move {
        tree.validate(
            &[],
            source_schema
                .root_schema
                .as_ref()
                .and_then(|s| s.value_schema.as_ref()),
            source_schema.root_schema.as_ref().map(|s| &s.schema_url),
            source_schema.root_schema.as_ref().map(|s| &s.definitions),
            &schema_context,
        )
        .await?;

        Ok(())
    }
    .boxed()
}
