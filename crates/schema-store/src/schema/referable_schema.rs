use futures::{future::BoxFuture, FutureExt};

use super::{
    AllOfSchema, AnyOfSchema, DocumentSchema, OneOfSchema, SchemaDefinitions, SchemaUrl,
    ValueSchema,
};

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

    pub fn resolve<'a: 'b, 'b>(
        &'a mut self,
        definitions: &'a SchemaDefinitions,
    ) -> BoxFuture<'b, Result<&'a ValueSchema, crate::Error>> {
        async move {
            match self {
                Referable::Ref {
                    reference,
                    title,
                    description,
                } => {
                    {
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
                            Err(crate::Error::DefinitionNotFound {
                                definition_ref: reference.clone(),
                            })?;
                        }
                    }
                    self.resolve(definitions).await
                }
                Referable::Resolved(resolved) => {
                    match resolved {
                        ValueSchema::OneOf(OneOfSchema { schemas, .. })
                        | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                        | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                            for schema in schemas.write().await.iter_mut() {
                                schema.resolve(definitions).await?;
                            }
                        }
                        _ => {}
                    }
                    Result::<&ValueSchema, crate::Error>::Ok(resolved)
                }
            }
        }
        .boxed()
    }

    #[allow(unused)]
    fn resolve_async<'a: 'b, 'b>(
        &'a mut self,
        document_schema: &'a DocumentSchema,
        schema_store: &'a crate::SchemaStore,
    ) -> BoxFuture<'b, Result<(&'a ValueSchema, Option<SchemaUrl>), crate::Error>> {
        Box::pin(async move {
            match self {
                Referable::Ref {
                    reference,
                    title,
                    description,
                } => {
                    let mut new_schema_url = None;
                    if let Some(definition_schema) = document_schema.definitions.get(reference) {
                        let mut referable_schema = definition_schema.to_owned();
                        if let Referable::Resolved(ref mut schema) = &mut referable_schema {
                            if title.is_some() || description.is_some() {
                                schema.set_title(title.to_owned());
                                schema.set_description(description.to_owned());
                            }
                        }

                        *self = referable_schema;
                    } else {
                        match reference.as_str() {
                            https_url if https_url.starts_with("https://") => {
                                if let Ok(schema_url) = SchemaUrl::parse(https_url) {
                                    if let Ok(document_schema) =
                                        schema_store.try_load_schema(&schema_url).await
                                    {
                                        if let Some(value_schema) = document_schema.value_schema {
                                            *self = Referable::Resolved(value_schema);
                                            new_schema_url = Some(schema_url);
                                        }
                                    }
                                }
                            }
                            http_url if http_url.starts_with("http://") => {
                                if let Ok(schema_url) = SchemaUrl::parse(http_url) {
                                    if let Ok(document_schema) =
                                        schema_store.try_load_schema(&schema_url).await
                                    {
                                        if let Some(value_schema) = document_schema.value_schema {
                                            *self = Referable::Resolved(value_schema);
                                            new_schema_url = Some(schema_url);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    self.resolve_async(document_schema, schema_store)
                        .await
                        .map(|(value_schema, _)| (value_schema, new_schema_url))
                }
                Referable::Resolved(resolved) => {
                    match resolved {
                        ValueSchema::OneOf(OneOfSchema { schemas, .. })
                        | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                        | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                            for schema in schemas.write().await.iter_mut() {
                                schema.resolve_async(document_schema, schema_store).await?;
                            }
                        }
                        _ => {}
                    }

                    Result::<(&ValueSchema, Option<SchemaUrl>), crate::Error>::Ok((resolved, None))
                }
            }
        })
    }
}
