use ast::AstNode;
use parser::parse_as;
use std::borrow::Cow;
use syntax::SyntaxElement;

use futures::FutureExt;
use itertools::Itertools;
use schema_store::{CurrentSchema, ValueSchema};

use crate::{change::Change, rule::array_values_order};

impl crate::Edit for ast::Array {
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
            tracing::error!("schema_url: {:?}", schema_url);
            tracing::error!("value_schema: {:?}", value_schema);

            if let (Some(schema_url), Some(value_schema), Some(definitions)) =
                (schema_url, value_schema, definitions)
            {
                changes.extend(
                    array_values_order(
                        self.values_with_comma().into_iter().collect_vec(),
                        &value_schema,
                        schema_context,
                    )
                    .await,
                );

                if let ValueSchema::Array(array_schema) = &value_schema {
                    if let Some(item_schema) = &array_schema.items {
                        if let Ok(Some(CurrentSchema {
                            schema_url,
                            value_schema,
                            definitions,
                        })) = item_schema
                            .write()
                            .await
                            .resolve(
                                Cow::Borrowed(schema_url),
                                Cow::Borrowed(definitions),
                                schema_context.store,
                            )
                            .await
                        {
                            for value in self.values() {
                                changes.extend(
                                    value
                                        .edit(
                                            accessors,
                                            Some(value_schema),
                                            Some(&schema_url),
                                            Some(&definitions),
                                            schema_context,
                                        )
                                        .await,
                                );
                            }
                        }
                    }
                }
            } else {
                for value in self.values() {
                    changes.extend(
                        value
                            .edit(accessors, None, None, None, schema_context)
                            .await,
                    );
                }
            }

            let values_with_comma = self.values_with_comma().into_iter().collect_vec();
            if let Some((value, None)) = values_with_comma.last() {
                if let Some(tailing_comment) = value.tailing_comment() {
                    let tailing_comment_with_comma = parse_as::<ast::Comma>(
                        &format!(",{}", tailing_comment.syntax().text()),
                        schema_context.toml_version,
                    )
                    .into_syntax_node()
                    .clone_for_update();

                    changes.push(Change::Remove {
                        target: SyntaxElement::Token(tailing_comment.syntax().clone()),
                    });
                    changes.push(Change::Append {
                        parent: SyntaxElement::Node(value.syntax().clone()),
                        new: SyntaxElement::Node(tailing_comment_with_comma),
                    });
                }
            }

            changes
        }
        .boxed()
    }
}
