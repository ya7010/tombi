use crate::completion::{
    find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    CompletionContent, CompletionEdit,
};

use super::{CompletionHint, FindCompletionContents};
use config::TomlVersion;
use schema_store::{Accessor, ArraySchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionContents for document_tree::Array {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match value_schema {
            Some(ValueSchema::Array(array)) => {
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);
                        if let Some(completion_items) = array.operate_item(
                            |item_schema| {
                                value.find_completion_contents(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    Some(item_schema),
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                    completion_hint,
                                )
                            },
                            definitions,
                        ) {
                            return completion_items;
                        }
                    }
                }
                Vec::with_capacity(0)
            }
            Some(ValueSchema::OneOf(one_of_schema)) => find_one_of_completion_items(
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
            Some(ValueSchema::AnyOf(any_of_schema)) => find_any_of_completion_items(
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
            Some(ValueSchema::AllOf(all_of_schema)) => find_all_if_completion_items(
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

impl FindCompletionContents for ArraySchema {
    fn find_completion_contents(
        &self,
        _accessors: &Vec<Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        _definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match completion_hint {
            Some(CompletionHint::InTableHeader) => Vec::with_capacity(0),
            _ => {
                let label = "[]".to_string();
                let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                vec![CompletionContent::new_type_hint_value(
                    label, edit, schema_url,
                )]
            }
        }
    }
}
