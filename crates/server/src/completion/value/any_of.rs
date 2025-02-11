use config::TomlVersion;
use futures::future::BoxFuture;
use futures::FutureExt;
use schema_store::{Accessor, SchemaDefinitions, Schemas};
use schema_store::{AnyOfSchema, SchemaUrl};

use crate::completion::{
    serde_value_to_completion_item, CompletionCandidate, CompletionContent, CompletionHint,
    CompositeSchemaImpl, FindCompletionContents,
};

impl CompositeSchemaImpl for AnyOfSchema {
    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn schemas(&self) -> &Schemas {
        &self.schemas
    }
}

pub fn find_any_of_completion_items<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a Vec<Accessor>,
    any_of_schema: &'a schema_store::AnyOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &'a [document_tree::Key],
    schema_url: Option<&'a SchemaUrl>,
    definitions: Option<&'a SchemaDefinitions>,
    schema_store: &'a schema_store::SchemaStore,
    completion_hint: Option<CompletionHint>,
) -> BoxFuture<'b, Vec<CompletionContent>>
where
    T: FindCompletionContents + Sync + Send,
{
    async move {
        let Some(definitions) = definitions else {
            unreachable!("definitions must be provided");
        };

        let mut completion_items = Vec::new();

        for referable_schema in any_of_schema.schemas.write().await.iter_mut() {
            if let Ok((value_schema, new_schema)) = referable_schema
                .resolve(definitions, schema_store)
                .await
            {
                let (schema_url, definitions) = if let Some((schema_url, definitions)) = &new_schema
                {
                    (Some(schema_url), Some(definitions))
                } else {
                    (schema_url, Some(definitions))
                };

                let schema_completions = value
                    .find_completion_contents(
                        accessors,
                        Some(value_schema),
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        &schema_store,
                        completion_hint,
                    )
                    .await;

                completion_items.extend(schema_completions);
            }
        }

        for completion_item in completion_items.iter_mut() {
            if completion_item.detail.is_none() {
                completion_item.detail = any_of_schema
                    .detail(definitions, &schema_store, completion_hint)
                    .await;
            }
            if completion_item.documentation.is_none() {
                completion_item.documentation = any_of_schema
                    .documentation(definitions, &schema_store, completion_hint)
                    .await;
            }
        }

        if let Some(default) = &any_of_schema.default {
            if let Some(completion_item) =
                serde_value_to_completion_item(default, position, schema_url, completion_hint)
            {
                completion_items.push(completion_item);
            }
        }

        completion_items
    }
    .boxed()
}
