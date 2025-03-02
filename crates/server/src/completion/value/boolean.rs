use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, BooleanSchema, SchemaDefinitions, SchemaUrl, ValueSchema};

use crate::completion::{
    completion_kind::CompletionKind, CompletionContent, CompletionEdit, CompletionHint,
    FindCompletionContents,
};

impl FindCompletionContents for BooleanSchema {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        position: text::Position,
        _keys: &'a [document_tree::Key],
        _accessors: &'a [Accessor],
        _value_schema: Option<&'a ValueSchema>,
        schema_url: Option<&'a SchemaUrl>,
        _definitions: Option<&'a SchemaDefinitions>,
        _schema_context: &'a schema_store::SchemaContext<'a>,
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
