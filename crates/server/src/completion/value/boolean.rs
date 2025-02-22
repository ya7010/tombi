use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{
    Accessor, BooleanSchema, SchemaDefinitions, SchemaStore, SchemaUrl, ValueSchema,
};

use crate::completion::{
    completion_kind::CompletionKind, CompletionContent, CompletionEdit, CompletionHint,
    FindCompletionContents,
};

impl FindCompletionContents for BooleanSchema {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a Vec<Accessor>,
        _value_schema: Option<&'a ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        _definitions: Option<&'a SchemaDefinitions>,
        _sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        _schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        async move {
            if let Some(enumerate) = &self.enumerate {
                enumerate
                    .iter()
                    .map(|value| {
                        let label = value.to_string();
                        let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                        CompletionContent::new_enumerate_value(
                            CompletionKind::Boolean,
                            value.to_string(),
                            edit,
                            schema_url,
                        )
                    })
                    .collect()
            } else {
                type_hint_boolean(position, schema_url, completion_hint)
            }
        }
        .boxed()
    }
}

pub fn type_hint_boolean(
    position: text::Position,
    schema_url: Option<&SchemaUrl>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    [true, false]
        .into_iter()
        .map(|value| {
            CompletionContent::new_type_hint_boolean(
                value,
                CompletionEdit::new_literal(&value.to_string(), position, completion_hint),
                schema_url,
            )
        })
        .collect()
}
