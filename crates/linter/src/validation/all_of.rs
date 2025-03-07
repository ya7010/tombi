use std::{borrow::Cow, fmt::Debug};

use futures::{future::BoxFuture, FutureExt};
use schema_store::CurrentSchema;

use super::Validate;

pub fn validate_all_of<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a [schema_store::SchemaAccessor],
    all_of_schema: &'a schema_store::AllOfSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a schema_store::SchemaContext<'a>,
) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>>
where
    T: Validate + Sync + Send + Debug,
{
    tracing::trace!("value = {:?}", value);
    tracing::trace!("all_of_schema = {:?}", all_of_schema);

    async move {
        let mut diagnostics = vec![];

        let mut schemas = all_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok(Some(CurrentSchema {
                value_schema,
                schema_url,
                definitions,
            })) = referable_schema
                .resolve(
                    Cow::Borrowed(schema_url),
                    Cow::Borrowed(definitions),
                    schema_context.store,
                )
                .await
            else {
                continue;
            };

            match value
                .validate(
                    accessors,
                    Some(value_schema),
                    Some(&schema_url),
                    Some(&definitions),
                    schema_context,
                )
                .await
            {
                Ok(()) => {}
                Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
    .boxed()
}
