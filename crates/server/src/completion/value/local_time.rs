use chrono;
use config::TomlVersion;
use schema_store::{Accessor, LocalTimeSchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{
    find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    CompletionHint, FindCompletionItems,
};

impl FindCompletionItems for document_tree::LocalTime {
    fn find_completion_items(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<tower_lsp::lsp_types::CompletionItem> {
        match value_schema {
            ValueSchema::LocalTime(local_time_schema) => local_time_schema.find_completion_items(
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

impl FindCompletionItems for LocalTimeSchema {
    fn find_completion_items(
        &self,
        _accessors: &Vec<Accessor>,
        _value_schema: &ValueSchema,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        _schema_url: Option<&Url>,
        _definitions: &SchemaDefinitions,
        _completion_hint: Option<CompletionHint>,
    ) -> Vec<tower_lsp::lsp_types::CompletionItem> {
        let mut completion_items = vec![];

        if let Some(enumerate) = &self.enumerate {
            for item in enumerate {
                completion_items.push(tower_lsp::lsp_types::CompletionItem {
                    label: item.to_string(),
                    kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
                    ..Default::default()
                });
            }
        }

        if let Some(default) = &self.default {
            completion_items.push(tower_lsp::lsp_types::CompletionItem {
                label: default.to_string(),
                kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
                ..Default::default()
            });
        }

        if completion_items.is_empty() {
            completion_items.push(tower_lsp::lsp_types::CompletionItem {
                label: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
                kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
                ..Default::default()
            })
        }

        completion_items
    }
}
