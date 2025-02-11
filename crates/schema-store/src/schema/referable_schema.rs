use futures::future::BoxFuture;

use super::{AllOfSchema, AnyOfSchema, OneOfSchema, SchemaDefinitions, SchemaUrl, ValueSchema};

#[derive(Debug, Clone, PartialEq)]
pub enum Referable<T> {
    Resolved(T),
    Ref {
        reference: String,
        title: Option<String>,
        description: Option<String>,
    },
}

impl<T> Referable<T> {
    pub fn resolved(&self) -> Option<&T> {
        match self {
            Self::Resolved(t) => Some(t),
            Self::Ref { .. } => None,
        }
    }
}

impl Referable<ValueSchema> {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        if let Some(ref_value) = object.get("$ref") {
            if let serde_json::Value::String(ref_string) = ref_value {
                return Some(Referable::Ref {
                    reference: ref_string.clone(),
                    title: object
                        .get("title")
                        .and_then(|title| title.as_str().map(|s| s.to_string())),
                    description: object
                        .get("description")
                        .and_then(|description| description.as_str().map(|s| s.to_string())),
                });
            }
        }

        ValueSchema::new(object).map(Referable::Resolved)
    }

    pub async fn value_type(&self) -> crate::ValueType {
        match self {
            Referable::Resolved(schema) => schema.value_type().await,
            Referable::Ref { .. } => unreachable!("unreachable ref value_tyle."),
        }
    }

    #[allow(unused)]
    pub fn resolve<'a: 'b, 'b>(
        &'a mut self,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a crate::SchemaStore,
    ) -> BoxFuture<
        'b,
        Result<(&'a ValueSchema, Option<(SchemaUrl, SchemaDefinitions)>), crate::Error>,
    > {
        Box::pin(async move {
            match self {
                Referable::Ref {
                    reference,
                    title,
                    description,
                } => {
                    let mut new_schema = None;
                    if let Some(definition_schema) = definitions.get(reference) {
                        let mut referable_schema = definition_schema.to_owned();
                        if let Referable::Resolved(ref mut schema) = &mut referable_schema {
                            if title.is_some() || description.is_some() {
                                schema.set_title(title.to_owned());
                                schema.set_description(description.to_owned());
                            }
                        }

                        *self = referable_schema;
                    } else {
                        if is_online_url(reference) {
                            let schema_url = SchemaUrl::parse(reference)?;
                            let document_schema =
                                schema_store.try_load_document_schema(&schema_url).await?;

                            if let Some(value_schema) = document_schema.value_schema {
                                *self = Referable::Resolved(value_schema);
                                new_schema =
                                    Some((schema_url, document_schema.definitions.clone()));
                            } else {
                                return Err(crate::Error::InvalidJsonSchemaReference {
                                    reference: reference.to_owned(),
                                });
                            }
                        } else {
                            return Err(crate::Error::UnsupportedReference {
                                reference: reference.to_owned(),
                            });
                        }
                    }

                    self.resolve(definitions, schema_store)
                        .await
                        .map(|(value_schema, _)| (value_schema, new_schema))
                }
                Referable::Resolved(resolved) => {
                    match resolved {
                        ValueSchema::OneOf(OneOfSchema { schemas, .. })
                        | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                        | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                            for schema in schemas.write().await.iter_mut() {
                                schema.resolve(definitions, schema_store).await?;
                            }
                        }
                        _ => {}
                    }

                    Result::<(&ValueSchema, _), _>::Ok((resolved, None))
                }
            }
        })
    }
}

pub fn is_online_url(reference: &str) -> bool {
    reference.starts_with("https://") || reference.starts_with("http://")
}
