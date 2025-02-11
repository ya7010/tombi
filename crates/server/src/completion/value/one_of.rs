use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, OneOfSchema, SchemaDefinitions, SchemaUrl, Schemas};

use crate::completion::{
    serde_value_to_completion_item, CompletionCandidate, CompletionContent, CompletionHint,
    CompositeSchemaImpl, FindCompletionContents,
};

impl CompositeSchemaImpl for OneOfSchema {
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

pub fn find_one_of_completion_items<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a Vec<Accessor>,
    one_of_schema: &'a schema_store::OneOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &'a [document_tree::Key],
    schema_url: Option<&'a SchemaUrl>,
    definitions: Option<&'a SchemaDefinitions>,
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

        for referable_schema in one_of_schema.schemas.write().await.iter_mut() {
            if let Ok(value_schema) = referable_schema.resolve(definitions).await {
                let schema_completions = value
                    .find_completion_contents(
                        accessors,
                        Some(value_schema),
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        Some(definitions),
                        completion_hint,
                    )
                    .await;

                completion_items.extend(schema_completions);
            }
        }

        for completion_item in completion_items.iter_mut() {
            if completion_item.detail.is_none() {
                completion_item.detail = one_of_schema.detail(definitions, completion_hint).await;
            }
            if completion_item.documentation.is_none() {
                completion_item.documentation = one_of_schema
                    .documentation(definitions, completion_hint)
                    .await;
            }
        }

        if let Some(default) = &one_of_schema.default {
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
