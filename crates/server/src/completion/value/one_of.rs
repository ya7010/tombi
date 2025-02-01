use config::TomlVersion;
use schema_store::{Accessor, OneOfSchema, SchemaDefinitions, Schemas};
use tower_lsp::lsp_types::Url;

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

pub fn find_one_of_completion_items<T>(
    value: &T,
    accessors: &Vec<Accessor>,
    one_of_schema: &schema_store::OneOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: Option<&SchemaDefinitions>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent>
where
    T: FindCompletionContents,
{
    let Some(definitions) = definitions else {
        unreachable!("definitions must be provided");
    };

    let mut completion_items = Vec::new();

    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(value_schema) = schema.resolve(definitions) {
                let schema_completions = value.find_completion_contents(
                    accessors,
                    Some(value_schema),
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    Some(definitions),
                    completion_hint,
                );

                completion_items.extend(schema_completions);
            }
        }
    }

    for completion_item in completion_items.iter_mut() {
        if completion_item.detail.is_none() {
            completion_item.detail = one_of_schema.detail(definitions, completion_hint);
        }
        if completion_item.documentation.is_none() {
            completion_item.documentation =
                one_of_schema.documentation(definitions, completion_hint);
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
