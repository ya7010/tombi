use super::{
    AllOfSchema, AnyOfSchema, DocumentSchema, OneOfSchema, SchemaDefinitions, ValueSchema,
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

    pub fn value_type(&self) -> crate::ValueType {
        match self {
            Referable::Resolved(schema) => schema.value_type(),
            Referable::Ref { .. } => unreachable!("unreachable ref value_tyle."),
        }
    }

    pub fn resolve<'a>(
        &'a mut self,
        definitions: &SchemaDefinitions,
    ) -> Result<&'a ValueSchema, crate::Error> {
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
                self.resolve(definitions)
            }
            Referable::Resolved(resolved) => {
                match resolved {
                    ValueSchema::OneOf(OneOfSchema { schemas, .. })
                    | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                    | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                        if let Ok(mut schemas) = schemas.write() {
                            for schema in schemas.iter_mut() {
                                schema.resolve(definitions)?;
                            }
                        }
                    }
                    _ => {}
                }
                Ok(resolved)
            }
        }
    }

    #[allow(unused)]
    async fn resolve_async<'a>(
        &'a mut self,
        document_schema: &'a DocumentSchema,
        schema_store: &'a crate::SchemaStore,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<&'a ValueSchema, crate::Error>> + 'a>,
    > {
        Box::pin(async move {
            match self {
                Referable::Ref {
                    reference,
                    title,
                    description,
                } => {
                    {
                        if let Some(definition_schema) = document_schema.definitions.get(reference)
                        {
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
                    self.resolve_async(document_schema, schema_store)
                        .await
                        .await
                }
                Referable::Resolved(resolved) => {
                    match resolved {
                        ValueSchema::OneOf(OneOfSchema { schemas, .. })
                        | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                        | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                            if let Ok(mut schemas) = schemas.write() {
                                for schema in schemas.iter_mut() {
                                    schema
                                        .resolve_async(document_schema, schema_store)
                                        .await
                                        .await?;
                                }
                            }
                        }
                        _ => {}
                    }
                    Result::<&ValueSchema, crate::Error>::Ok(resolved)
                }
            }
        })
    }
}
