use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaDefinitions, SchemaUrl, StringSchema, ValueSchema};

use crate::completion::{
    completion_kind::CompletionKind, CompletionContent, CompletionEdit, CompletionHint,
    FindCompletionContents,
};

impl FindCompletionContents for StringSchema {
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
            let mut completion_items = vec![];

            if let Some(default) = &self.default {
                let label = format!("\"{default}\"");
                let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                completion_items.push(CompletionContent::new_default_value(
                    CompletionKind::String,
                    label,
                    edit,
                    schema_url,
                ));
            }

            if let Some(enumerate) = &self.enumerate {
                for item in enumerate {
                    let label = format!("\"{item}\"");
                    let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                    completion_items.push(CompletionContent::new_enumerate_value(
                        CompletionKind::String,
                        label,
                        edit,
                        schema_url,
                    ));
                }
                return completion_items;
            }

            completion_items.extend(
                type_hint_string(position, schema_url, completion_hint)
                    .into_iter()
                    .filter(|completion_content| {
                        self.default
                            .as_ref()
                            .map(|default| default != &completion_content.label)
                            .unwrap_or(true)
                    }),
            );

            completion_items
        }
        .boxed()
    }
}

pub fn type_hint_string(
    position: text::Position,
    schema_url: Option<&SchemaUrl>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    [('\"', "BasicString"), ('\'', "LiteralString")]
        .into_iter()
        .map(|(quote, detail)| {
            CompletionContent::new_type_hint_string(
                CompletionKind::String,
                quote,
                detail,
                CompletionEdit::new_string_literal(quote, position, completion_hint),
                schema_url,
            )
        })
        .collect()
}
