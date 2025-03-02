use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, OffsetDateTimeSchema, SchemaDefinitions, SchemaUrl, ValueSchema};

use crate::completion::{
    completion_kind::CompletionKind, CompletionContent, CompletionEdit, CompletionHint,
    FindCompletionContents,
};

impl FindCompletionContents for OffsetDateTimeSchema {
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

            if let Some(enumerate) = &self.enumerate {
                for item in enumerate {
                    let label = item.to_string();
                    let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                    completion_items.push(CompletionContent::new_enumerate_value(
                        CompletionKind::OffsetDateTime,
                        label,
                        edit,
                        schema_url,
                    ));
                }
            }

            if let Some(default) = &self.default {
                let label = default.to_string();
                let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                completion_items.push(CompletionContent::new_default_value(
                    CompletionKind::OffsetDateTime,
                    label,
                    edit,
                    schema_url,
                ));
            }

            if completion_items.is_empty() {
                completion_items.extend(type_hint_offset_date_time(
                    position,
                    schema_url,
                    completion_hint,
                ));
            }

            completion_items
        }
        .boxed()
    }
}

pub fn type_hint_offset_date_time(
    position: text::Position,
    schema_url: Option<&SchemaUrl>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    let mut today = chrono::Local::now();
    if let Some(time) = chrono::NaiveTime::from_hms_opt(0, 0, 0) {
        today = match today.with_time(time) {
            chrono::LocalResult::Single(today) => today,
            _ => today,
        };
    };
    let label = today.format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string();
    let edit = CompletionEdit::new_selectable_literal(&label, position, completion_hint);

    vec![CompletionContent::new_type_hint_value(
        CompletionKind::OffsetDateTime,
        label,
        "OffsetDateTime",
        edit,
        schema_url,
    )]
}
