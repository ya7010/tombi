use crate::completion::{
    completion_kind::CompletionKind, schema_completion::SchemaCompletion, CompletionContent,
    CompletionEdit,
};

use super::{
    all_of::find_all_if_completion_items, any_of::find_any_of_completion_items,
    one_of::find_one_of_completion_items, CompletionHint, FindCompletionContents,
};
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
        definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match value_schema {
            Some(ValueSchema::Array(array_schema)) => {
                let Some(definitions) = definitions else {
                    unreachable!("definitions must be provided");
                };

                let mut new_item_index = 0;
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().end() < position {
                        new_item_index = index + 1;
                    }
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);
                        if let Some(completion_items) = array_schema.operate_item(
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
                                    Some(definitions),
                                    completion_hint,
                                )
                            },
                            definitions,
                        ) {
                            return completion_items;
                        }
                    }
                }
                if let Some(completion_items) = array_schema.operate_item(
                    |item_schema| {
                        SchemaCompletion.find_completion_contents(
                            &accessors
                                .clone()
                                .into_iter()
                                .chain(std::iter::once(Accessor::Index(new_item_index)))
                                .collect(),
                            Some(item_schema),
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            Some(definitions),
                            completion_hint,
                        )
                    },
                    definitions,
                ) {
                    return completion_items;
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
            Some(_) => Vec::with_capacity(0),
            None => {
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);
                        return value.find_completion_contents(
                            &accessors
                                .clone()
                                .into_iter()
                                .chain(std::iter::once(accessor))
                                .collect(),
                            None,
                            toml_version,
                            position,
                            keys,
                            None,
                            None,
                            completion_hint,
                        );
                    }
                }
                Vec::with_capacity(0)
            }
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
        _definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match completion_hint {
            Some(CompletionHint::InTableHeader) => Vec::with_capacity(0),
            _ => type_hint_array(position, schema_url, completion_hint),
        }
    }
}

pub fn type_hint_array(
    position: text::Position,
    schema_url: Option<&Url>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    let edit = CompletionEdit::new_array_literal(position, completion_hint);

    vec![CompletionContent::new_type_hint_value(
        CompletionKind::Array,
        "[]",
        "Array",
        edit,
        schema_url,
    )]
}
