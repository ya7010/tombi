use document_tree::TryIntoDocumentTree;
use futures::FutureExt;
use itertools::Itertools;
use schema_store::SchemaAccessor;

use super::get_schema;

impl crate::Edit for ast::KeyValue {
    fn edit<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move {
            let mut changes = vec![];

            let Some(keys) = self.keys() else {
                return changes;
            };

            let keys_accessors = keys
                .keys()
                .filter_map(|key| {
                    key.try_to_raw_text(schema_context.toml_version)
                        .ok()
                        .map(SchemaAccessor::Key)
                })
                .collect_vec();

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                if let Some(value_schema) = get_schema(
                    &document_tree::Value::Table(
                        self.clone()
                            .try_into_document_tree(schema_context.toml_version)
                            .unwrap(),
                    ),
                    &keys_accessors.clone(),
                    value_schema,
                    schema_url,
                    definitions,
                    schema_context,
                )
                .await
                {
                    if let Some(value) = self.value() {
                        changes.extend(
                            value
                                .edit(
                                    &accessors
                                        .to_vec()
                                        .into_iter()
                                        .chain(keys_accessors.into_iter())
                                        .collect_vec(),
                                    Some(&value_schema),
                                    Some(&schema_url),
                                    Some(&definitions),
                                    schema_context,
                                )
                                .await,
                        );
                    }
                }
            } else {
                if let Some(value) = self.value() {
                    changes.extend(
                        value
                            .edit(accessors, None, None, None, schema_context)
                            .await,
                    );
                }
            }

            changes
        }
        .boxed()
    }
}
