use crate::completion::{
    find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    CompletionContent,
};

use super::{CompletionHint, FindCompletionContents};
use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionContents for document_tree::Array {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match value_schema {
            ValueSchema::Array(array) => {
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);

                        if let Some(items) = &array.items {
                            if let Ok(mut item_schema) = items.write() {
                                let Ok(value_schema) = item_schema.resolve(definitions) else {
                                    continue;
                                };

                                let mut completion_contents = value.find_completion_contents(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    value_schema,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    completion_hint,
                                );
                                for completion_content in &mut completion_contents {
                                    if completion_content.detail.is_none()
                                        && completion_content.documentation.is_none()
                                    {
                                        if let Some(title) = &array.title {
                                            completion_content.detail = Some(title.clone());
                                        }
                                        if let Some(description) = &array.description {
                                            completion_content.documentation =
                                                Some(description.clone());
                                        }
                                    }
                                }
                                return completion_contents;
                            }
                        }

                        return Vec::with_capacity(0);
                    }
                }
                Vec::with_capacity(0)
            }
            ValueSchema::OneOf(one_of_schema) => find_one_of_completion_items(
                self,
                accessors,
                one_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AnyOf(any_of_schema) => find_any_of_completion_items(
                self,
                accessors,
                any_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AllOf(all_of_schema) => find_all_if_completion_items(
                self,
                accessors,
                all_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            _ => Vec::with_capacity(0),
        }
    }
}
