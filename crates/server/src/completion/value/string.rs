use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, StringSchema, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{
    find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    CompletionContent, CompletionHint, FindCompletionContents,
};

impl FindCompletionContents for document_tree::String {
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
            ValueSchema::String(string_schema) => string_schema.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
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

impl FindCompletionContents for StringSchema {
    fn find_completion_contents(
        &self,
        _accessors: &Vec<Accessor>,
        _value_schema: &ValueSchema,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        _definitions: &SchemaDefinitions,
        _completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        let mut completion_items = vec![];

        if let Some(enumerate) = &self.enumerate {
            for value in enumerate {
                completion_items.push(CompletionContent {
                    label: format!("\"{value}\""),
                    schema_url: schema_url.cloned(),
                    ..Default::default()
                });
            }
        }

        if let Some(default) = &self.default {
            completion_items.push(CompletionContent::new_default_value(
                format!("\"{default}\""),
                schema_url,
            ));
        }

        completion_items
    }
}
