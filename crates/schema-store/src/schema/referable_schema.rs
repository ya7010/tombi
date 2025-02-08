use super::{AllOfSchema, AnyOfSchema, OneOfSchema, SchemaDefinitions, ValueSchema};

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
}
